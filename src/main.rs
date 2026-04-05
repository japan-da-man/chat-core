mod shared;

use anyhow::Result;
use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use shared::config::Config;
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;
use tracing_subscriber::EnvFilter;

#[derive(Clone)]
struct AppState {
    job_tx: mpsc::Sender<Job>,
}

#[derive(Debug)]
enum Job {
    Example { message: String },
}

#[derive(Debug, Deserialize)]
struct EnqueueRequest {
    message: String,
}

#[derive(Debug, Serialize)]
struct EnqueueResponse {
    accepted: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::load()?;

    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .or_else(|_| {
                    EnvFilter::try_new(
                        config
                            .rust_log
                            .clone()
                            .unwrap_or_else(|| "info".to_string()),
                    )
                })?,
        )
        .init();

    let shutdown = CancellationToken::new();

    let (job_tx, job_rx) = mpsc::channel::<Job>(100);

    let worker_token = shutdown.clone();
    let worker_handle = tokio::spawn(async move {
        worker_loop(job_rx, worker_token).await;
        cleanup().await;
    });

    let signal_token = shutdown.clone();
    let signal_handle = tokio::spawn(async move {
        shutdown_signal().await;
        signal_token.cancel();
    });

    let state = AppState { job_tx };

    let app = Router::new()
        .route("/", get(|| async { "Hello, rust!" }))
        .route("/axum", get(|| async { "Hello, axum!" }))
        .route("/jobs", post(enqueue_job))
        .with_state(state);

    let addr = config.bind_addr();
    tracing::info!("starting server: http://{}", addr);

    let server_token = shutdown.clone();
    let listener = tokio::net::TcpListener::bind(addr).await?;
    let serve_result = axum::serve(listener, app)
        .with_graceful_shutdown(async move {
            server_token.cancelled().await;
        })
        .await;

    shutdown.cancel();

    if let Err(err) = signal_handle.await {
        tracing::error!("signal task join error: {err}");
    }

    if let Err(err) = worker_handle.await {
        tracing::error!("worker task join error: {err}");
    }

    serve_result?;

    Ok(())
}

async fn enqueue_job(
    State(state): State<AppState>,
    Json(req): Json<EnqueueRequest>,
) -> Result<(StatusCode, Json<EnqueueResponse>), StatusCode> {
    let job = Job::Example {
        message: req.message,
    };

    state
        .job_tx
        .send(job)
        .await
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;

    Ok((
        StatusCode::ACCEPTED,
        Json(EnqueueResponse { accepted: true }),
    ))
}

async fn worker_loop(mut job_rx: mpsc::Receiver<Job>, shutdown: CancellationToken) {
    loop {
        tokio::select! {
            _ = shutdown.cancelled() => {
                tracing::info!("worker: shutdown requested");
                break;
            }

            maybe_job = job_rx.recv() => {
                match maybe_job {
                    Some(job) => {
                        if let Err(err) = process_job(job, &shutdown).await {
                            tracing::error!("worker: failed to process job: {err:#}");
                        }
                    }
                    None => {
                        tracing::info!("worker: all senders dropped, stopping worker");
                        break;
                    }
                }
            }
        }
    }
}

async fn process_job(job: Job, shutdown: &CancellationToken) -> Result<()> {
    match job {
        Job::Example { message } => {
            tracing::info!("worker: received job: {}", message);

            tokio::select! {
                _ = shutdown.cancelled() => {
                    tracing::info!("worker: cancelled before processing job");
                    Ok(())
                }
                _ = tokio::time::sleep(std::time::Duration::from_secs(2)) => {
                    // TODO: 実際の業務処理に置き換える
                    tracing::info!("worker: finished job");
                    Ok(())
                }
            }
        }
    }
}

async fn cleanup() {
    tracing::info!("worker: cleanup started");

    // TODO: cleanup処理を追加する

    tracing::info!("worker: cleanup finished");
}

#[derive(Debug)]
enum ShutdownReason {
    CtrlC,
    Sigterm,
}

async fn shutdown_signal() {
    match shutdown_signal_inner().await {
        Ok(reason) => {
            tracing::info!("shutdown signal received: {:?}", reason);
        }
        Err(err) => {
            tracing::error!("failed to listen for shutdown signal: {err}");
        }
    }
}

async fn shutdown_signal_inner() -> std::io::Result<ShutdownReason> {
    use tokio::signal;

    #[cfg(unix)]
    {
        use tokio::signal::unix::{signal as unix_signal, SignalKind};

        let mut sigterm = unix_signal(SignalKind::terminate())?;

        tokio::select! {
            res = signal::ctrl_c() => {
                res?;
                Ok(ShutdownReason::CtrlC)
            }
            _ = sigterm.recv() => {
                Ok(ShutdownReason::Sigterm)
            }
        }
    }

    #[cfg(not(unix))]
    {
        signal::ctrl_c().await?;
        Ok(ShutdownReason::CtrlC)
    }
}
