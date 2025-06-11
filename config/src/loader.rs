// finalverse-config/src/loader.rs

use crate::{FinalverseConfig, ConfigError, Result};
use std::fs;
use std::path::Path;

pub struct ConfigLoader;

impl ConfigLoader {
    /// Load configuration from a TOML file
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<FinalverseConfig> {
        let contents = fs::read_to_string(&path)?;
        Self::load_from_string(&contents)
    }
    
    /// Load configuration from a TOML string
    pub fn load_from_string(contents: &str) -> Result<FinalverseConfig> {
        let config: FinalverseConfig = toml::from_str(contents)?;
        Ok(config)
    }
    
    /// Load configuration from multiple files (for environment-specific overrides)
    pub fn load_with_overrides<P: AsRef<Path>>(base_path: P, override_paths: Vec<P>) -> Result<FinalverseConfig> {
        let mut config = Self::load_from_file(base_path)?;
        
        for path in override_paths {
            if path.as_ref().exists() {
                let override_config = Self::load_from_file(path)?;
                config = Self::merge_configs(config, override_config);
            }
        }
        
        Ok(config)
    }
    
    /// Merge two configurations, with the second overriding the first
    fn merge_configs(base: FinalverseConfig, override_config: FinalverseConfig) -> FinalverseConfig {
        // This is a simplified merge - in production, you'd want a more sophisticated merge strategy
        // For now, we just return the override config
        // TODO: Implement proper deep merge
        override_config
    }
    
    /// Generate a sample configuration file
    pub fn generate_sample_config() -> String {
        let sample = FinalverseConfig::default();
        toml::to_string_pretty(&sample).unwrap()
    }
    
    /// Save configuration to a file
    pub fn save_to_file<P: AsRef<Path>>(config: &FinalverseConfig, path: P) -> Result<()> {
        let contents = toml::to_string_pretty(config)
            .map_err(|e| ConfigError::Validation(format!("Failed to serialize config: {}", e)))?;
        fs::write(path, contents)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_load_from_string() {
        let config_str = r#"
[general]
server_name = "Test Server"
version = "1.0.0"
environment = "development"
debug_mode = true
log_level = "debug"
log_format = "json"

[network]
host = "127.0.0.1"
port = 9090
websocket_port = 9091
grpc_port = 50052
public_url = "http://localhost:9090"
cors_origins = ["*"]
max_connections = 5000
connection_timeout_secs = 30
        "#;
        
        let config = ConfigLoader::load_from_string(config_str).unwrap();
        assert_eq!(config.general.server_name, "Test Server");
        assert_eq!(config.network.port, 9090);
        assert!(config.general.debug_mode);
    }
    
    #[test]
    fn test_generate_sample_config() {
        let sample = ConfigLoader::generate_sample_config();
        assert!(sample.contains("[general]"));
        assert!(sample.contains("[network]"));
        assert!(sample.contains("[ai]"));
    }
}