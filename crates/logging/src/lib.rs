use std::sync::Once;
use tracing_subscriber::{fmt, EnvFilter};
use finalverse_config::{load_default_config, FinalverseConfig};

static INIT: Once = Once::new();

/// Initialize global logging subscriber.
///
/// `level` can override the default log level. If `None`, the
/// `FINALVERSE_LOG_LEVEL` or `RUST_LOG` env vars are used, defaulting to `info`.
/// The log format is chosen based on `FinalverseConfig::general.log_format`,
/// falling back to `text` if configuration loading fails.
pub fn init(level: Option<&str>) {
    INIT.call_once(|| {
        let config: Option<FinalverseConfig> = load_default_config().ok();
        let log_level = level
            .map(|s| s.to_string())
            .or_else(|| std::env::var("FINALVERSE_LOG_LEVEL").ok())
            .or_else(|| std::env::var("RUST_LOG").ok())
            .unwrap_or_else(|| "info".to_string());
        let env_filter = EnvFilter::new(log_level);

        let log_format = config
            .as_ref()
            .map(|c| c.general.log_format.as_str())
            .unwrap_or("text");

        let subscriber_builder = fmt().with_env_filter(env_filter);
        match log_format {
            "json" => subscriber_builder.json().init(),
            "pretty" => subscriber_builder.pretty().init(),
            _ => subscriber_builder.init(),
        }
    });
}

