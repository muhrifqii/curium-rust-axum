// inspired from loco logger

use std::sync::OnceLock;

use serde::Deserialize;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{
    EnvFilter, Layer, Registry,
    fmt::{self, MakeWriter},
    layer::SubscriberExt as _,
    util::SubscriberInitExt as _,
};

use crate::{
    infrastructure::config,
    types::{AppError, AppResult},
};

static NONBLOCKING_WORK_GUARD_KEEP: OnceLock<WorkerGuard> = OnceLock::new();

#[derive(Debug, Default, Clone, Deserialize)]
pub enum LogRotation {
    #[serde(rename = "minutely")]
    Minutely,
    #[serde(rename = "hourly")]
    Hourly,
    #[serde(rename = "daily")]
    Daily,
    #[serde(rename = "never")]
    #[default]
    Never,
}

#[derive(Debug, Default, Clone, Deserialize)]
pub enum LogLevel {
    #[serde(rename = "off")]
    Off,
    #[serde(rename = "trace")]
    Trace,
    #[serde(rename = "debug")]
    Debug,
    #[serde(rename = "info")]
    #[default]
    Info,
    #[serde(rename = "warn")]
    Warn,
    #[serde(rename = "error")]
    Error,
}

#[derive(Debug, Default, Clone, Deserialize)]
pub enum LogFormat {
    #[serde(rename = "compact")]
    #[default]
    Compact,
    #[serde(rename = "pretty")]
    Pretty,
    #[serde(rename = "json")]
    Json,
}

pub fn init(config: &config::LoggerConfig) -> AppResult<()> {
    let mut layers: Vec<Box<dyn Layer<Registry> + Sync + Send>> = Vec::new();

    if let Some(file_appender_config) = config.file.as_ref() {
        if file_appender_config.enable {
            let dir = file_appender_config
                .dir
                .as_ref()
                .map_or_else(|| "./logs".to_string(), ToString::to_string);

            let mut rolling_builder = tracing_appender::rolling::Builder::default()
                .max_log_files(file_appender_config.max_log_files);

            rolling_builder = match file_appender_config.rotation {
                LogRotation::Minutely => {
                    rolling_builder.rotation(tracing_appender::rolling::Rotation::MINUTELY)
                }
                LogRotation::Hourly => {
                    rolling_builder.rotation(tracing_appender::rolling::Rotation::HOURLY)
                }
                LogRotation::Daily => {
                    rolling_builder.rotation(tracing_appender::rolling::Rotation::DAILY)
                }
                LogRotation::Never => {
                    rolling_builder.rotation(tracing_appender::rolling::Rotation::NEVER)
                }
            };

            let file_appender = rolling_builder
                .filename_suffix("backend.log")
                .build(dir)
                .map_err(AppError::map)?;

            let file_appender_layer = if file_appender_config.non_blocking {
                let (non_blocking_file_appender, work_guard) =
                    tracing_appender::non_blocking(file_appender);
                NONBLOCKING_WORK_GUARD_KEEP
                    .set(work_guard)
                    .map_err(|_| AppError::string("cannot lock for appender"))?;
                init_layer(
                    non_blocking_file_appender,
                    &file_appender_config.format,
                    false,
                )
            } else {
                init_layer(file_appender, &file_appender_config.format, false)
            };
            layers.push(file_appender_layer);
        }
    }

    if config.stdout {
        let stdout_layer = init_layer(std::io::stdout, &config.format, true);
        layers.push(stdout_layer);
    }

    if !layers.is_empty() {
        tracing_subscriber::registry()
            .with(layers)
            .with(EnvFilter::from_default_env())
            .init();
    }
    Ok(())
}

fn init_layer<W2>(
    make_writer: W2,
    format: &LogFormat,
    ansi: bool,
) -> Box<dyn Layer<Registry> + Sync + Send>
where
    W2: for<'writer> MakeWriter<'writer> + Sync + Send + 'static,
{
    match format {
        LogFormat::Compact => fmt::Layer::default()
            .with_ansi(ansi)
            .with_writer(make_writer)
            .compact()
            .boxed(),
        LogFormat::Pretty => fmt::Layer::default()
            .with_ansi(ansi)
            .with_writer(make_writer)
            .pretty()
            .boxed(),
        LogFormat::Json => fmt::Layer::default()
            .with_ansi(ansi)
            .with_writer(make_writer)
            .json()
            .boxed(),
    }
}
