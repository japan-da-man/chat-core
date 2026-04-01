# 01 Project Bootstrap

## 目的

Rust プロジェクトの土台を作り、以降の schema / command / query 実装を進められる状態にする。

## やること

- [ ] Rust プロジェクトを初期化する
- [ ] crate は単一 package で開始する
- [ ] `src/` 配下に次のディレクトリを作る
- [ ] `src/presentation`
- [ ] `src/application`
- [ ] `src/domain`
- [ ] `src/infrastructure`
- [ ] `src/shared`
- [ ] `tests/` ディレクトリを作る
- [ ] `migrations/` ディレクトリを作る
- [ ] `Cargo.toml` に MVP に必要な依存を追加する
- [ ] `axum`
- [ ] `tokio`
- [ ] `tower`
- [ ] `tower-http`
- [ ] `sqlx`
- [ ] `serde`
- [ ] `tracing`
- [ ] `tracing-subscriber`
- [ ] `thiserror`
- [ ] `anyhow`
- [ ] `validator` または独自バリデーション方針を決める
- [ ] `uuid` または `ulid` を追加する
- [ ] 環境変数を読む設定モジュールを作る
- [ ] `APP_HOST`
- [ ] `APP_PORT`
- [ ] `DATABASE_URL`
- [ ] `INTERNAL_API_BEARER_TOKEN`
- [ ] `RUST_LOG`
- [ ] `APP_ENV`
- [ ] `SEARCH_BACKEND`
- [ ] `main.rs` でアプリ起動、router 初期化、graceful shutdown の土台を書く
- [ ] request id / tracing / CORS / compression の middleware 配置方針を決める
- [ ] `cargo fmt`、`cargo clippy`、`cargo test` を実行しやすい状態にする

## 実装メモ

- `chat-core` は API サーバー兼 WebSocket サーバーとして単一プロセスで開始する
- 初期段階では REST と WebSocket だけを扱い、gRPC は入れない
- `tower-http` は最低でも trace, cors, compression, request-id を候補にする
- `sqlx` の feature は PostgreSQL と migration 実行に必要なものだけに絞る
- crate version はこの段階で pin する

## 完了条件

- `cargo run` で空の HTTP サーバーが起動する
- `cargo fmt` と `cargo clippy` が通る
- `src/` と `tests/` の配置方針が固まっている
- 環境変数の読み口が 1 箇所にまとまっている
