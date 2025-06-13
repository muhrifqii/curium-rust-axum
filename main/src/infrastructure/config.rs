use figment::{
    Figment,
    providers::{Env, Format, Yaml},
};
use serde::Deserialize;

use crate::infrastructure::logger;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub db: DbConfig,
    pub server: ServerConfig,
    pub logger: LoggerConfig,
    pub jwt: Option<JWTConfig>,
}

impl Config {
    /// Load configuration based on the `PROFILE` environment variable ("dev" or "prod").
    /// Falls back to "dev" if `PROFILE` is not set.
    /// Returns tuple of (Config, is_production)
    pub fn from_profile() -> (Self, bool) {
        // Determine profile: "dev" or "prod"
        let profile =
            std::env::var("APP_ENV_PROFILE").unwrap_or_else(|_| "dev".into());

        let is_prod = profile == "prod" || profile == "production";
        if !is_prod {
            unsafe { std::env::set_var("RUST_BACKTRACE", "1") };
        }

        let config_path = format!("configs/{}.yaml", profile);
        let config = Figment::new()
            .merge(Yaml::file(config_path))
            .merge(Env::raw())
            .extract()
            .expect("Config setup failed");
        (config, is_prod)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub port: u32,
    pub host: String,
    pub prefix: Option<String>,
    /// Identify via the `Server` header
    pub ident: Option<String>,
}

impl ServerConfig {
    #[must_use]
    pub fn url(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct JWTConfig {
    pub secret: String,
    pub expiration: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LoggerConfig {
    pub stdout: bool,
    pub level: logger::LogLevel,
    pub format: logger::LogFormat,
    pub file: Option<LoggerFileAppenderConfig>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LoggerFileAppenderConfig {
    pub enable: bool,
    pub level: logger::LogLevel,
    #[serde(default)]
    pub non_blocking: bool,
    pub format: logger::LogFormat,
    pub rotation: logger::LogRotation,
    pub dir: Option<String>,
    pub max_log_files: usize,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PgDbConfig {
    pub host: String,
    pub port: u32,
    pub user: String,
    pub password: String,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DbConfig {
    pub pg: Option<PgDbConfig>,
    pub sqlite: Option<String>,
}

impl DbConfig {
    #[must_use]
    pub fn url(&self) -> String {
        if let Some(path) = self.sqlite.as_ref() {
            return format!("sqlite://{}", path);
        } else if let Some(pg) = self.pg.as_ref() {
            return format!(
                "postgres://{}:{}@{}:{}/{}?sslmode=disable",
                pg.user, pg.password, pg.host, pg.port, pg.name
            );
        }
        String::new()
    }
}
