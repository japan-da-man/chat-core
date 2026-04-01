use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AppEnv {
    Local,
    Dev,
    Prod,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SearchBackend {
    Postgres,
    OpenSearch,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub app_host: String,
    pub app_port: u16,
    pub database_url: String,
    pub internal_api_bearer_token: String,
    pub app_env: AppEnv,
    pub search_backend: SearchBackend,
    pub rust_log: Option<String>,
}

impl Config {
    pub fn load() -> anyhow::Result<Self> {
        let _ = dotenvy::dotenv();
        let config = envy::from_env::<Config>()?;
        Ok(config)
    }

    pub fn bind_addr(&self) -> String {
        format!("{}:{}", self.app_host, self.app_port)
    }
}
