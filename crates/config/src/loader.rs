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
        use toml::Value;

        /// Recursively merge two `toml::Value` structures.
        fn merge_value(base: &mut Value, overlay: Value) {
            match overlay {
                Value::Table(overlay_table) => {
                    if let Value::Table(base_table) = base {
                        for (k, v) in overlay_table {
                            match base_table.get_mut(&k) {
                                Some(base_val) => merge_value(base_val, v),
                                None => {
                                    base_table.insert(k, v);
                                }
                            }
                        }
                    } else {
                        *base = Value::Table(overlay_table);
                    }
                }
                v => {
                    *base = v;
                }
            }
        }

        // Convert both configs to `toml::Value` so we can merge recursively
        let mut base_val = Value::try_from(base).expect("failed to serialize base config");
        let overlay_val = Value::try_from(override_config).expect("failed to serialize override config");

        merge_value(&mut base_val, overlay_val);

        base_val.try_into().expect("failed to deserialize merged config")
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
api_port = 9090
realtime_port = 9091
metrics_port = 9001
public_api_url = "http://localhost:9090"
public_realtime_url = "ws://localhost:9091"
cors_origins = ["*"]
max_connections = 5000
connection_timeout_secs = 30
        "#;
        
        let config = ConfigLoader::load_from_string(config_str).unwrap();
        assert_eq!(config.general.server_name, "Test Server");
        assert_eq!(config.network.api_port, 9090);
        assert!(config.general.debug_mode);
    }
    
    #[test]
    fn test_generate_sample_config() {
        let sample = ConfigLoader::generate_sample_config();
        assert!(sample.contains("[general]"));
        assert!(sample.contains("[network]"));
        assert!(sample.contains("[ai]"));
    }

    #[test]
    fn test_merge_configs_overrides_primitives() {
        let mut base = FinalverseConfig::default();
        base.general.server_name = "Base".to_string();

        let mut overlay = FinalverseConfig::default();
        overlay.general.server_name = "Override".to_string();
        overlay.network.api_port = 9000;

        let merged = ConfigLoader::merge_configs(base.clone(), overlay);

        assert_eq!(merged.general.server_name, "Override");
        assert_eq!(merged.network.api_port, 9000);
        // Unchanged field from base
        assert_eq!(merged.network.realtime_port, base.network.realtime_port);
    }

    #[test]
    fn test_merge_configs_nested_maps() {
        let base = FinalverseConfig::default();

        let mut overlay = FinalverseConfig::default();
        overlay.grpc_services.services.clear();
        overlay
            .grpc_services
            .services
            .insert("new-service".to_string(), "127.0.0.1:60000".parse().unwrap());

        let merged = ConfigLoader::merge_configs(base.clone(), overlay);

        // Base services remain
        assert!(merged.grpc_services.services.contains_key("song-engine"));
        // New service added
        assert!(merged.grpc_services.services.contains_key("new-service"));
    }
}
