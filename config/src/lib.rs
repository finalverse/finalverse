// finalverse-config/src/lib.rs

pub mod config;
pub mod loader;
pub mod validator;
pub mod environment;

pub use config::*;
pub use loader::ConfigLoader;
pub use validator::ConfigValidator;
pub use environment::apply_env_overrides;

use serde::{Deserialize, Serialize};
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Parse error: {0}")]
    Parse(#[from] toml::de::Error),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Environment variable error: {0}")]
    Environment(String),
}

pub type Result<T> = std::result::Result<T, ConfigError>;

/// Main entry point for loading and validating configuration
pub fn load_config<P: AsRef<Path>>(path: P) -> Result<FinalverseConfig> {
    let mut config = ConfigLoader::load_from_file(path)?;

    // Apply environment variable overrides
    apply_env_overrides(&mut config)?;

    // Validate the configuration
    ConfigValidator::validate(&config)?;

    Ok(config)
}

/// Load configuration with default path
pub fn load_default_config() -> Result<FinalverseConfig> {
    let config_path = std::env::var("FINALVERSE_CONFIG")
        .unwrap_or_else(|_| "config.toml".to_string());

    load_config(config_path)
}