// finalverse-config/src/validator.rs

use crate::{FinalverseConfig, ConfigError, Result};
use std::collections::HashSet;

pub struct ConfigValidator;

impl ConfigValidator {
    /// Validate the entire configuration
    pub fn validate(config: &FinalverseConfig) -> Result<()> {
        Self::validate_general(&config.general)?;
        Self::validate_network(&config.network)?;
        Self::validate_services(&config.services)?;
        Self::validate_ai(&config.ai)?;
        Self::validate_database(&config.database)?;
        Self::validate_cache(&config.cache)?;
        Self::validate_security(&config.security)?;
        Self::validate_performance(&config.performance)?;
        Self::validate_monitoring(&config.monitoring)?;
        Self::validate_game(&config.game)?;
        
        Ok(())
    }
    
    fn validate_general(general: &crate::config::GeneralConfig) -> Result<()> {
        if general.server_name.is_empty() {
            return Err(ConfigError::Validation("Server name cannot be empty".to_string()));
        }
        
        let valid_log_levels = ["trace", "debug", "info", "warn", "error"];
        if !valid_log_levels.contains(&general.log_level.as_str()) {
            return Err(ConfigError::Validation(
                format!("Invalid log level: {}. Must be one of: {:?}", general.log_level, valid_log_levels)
            ));
        }
        
        let valid_log_formats = ["json", "text", "pretty"];
        if !valid_log_formats.contains(&general.log_format.as_str()) {
            return Err(ConfigError::Validation(
                format!("Invalid log format: {}. Must be one of: {:?}", general.log_format, valid_log_formats)
            ));
        }
        
        Ok(())
    }
    
    fn validate_network(network: &crate::config::NetworkConfig) -> Result<()> {
        if network.api_port == 0 {
            return Err(ConfigError::Validation("API port cannot be 0".to_string()));
        }
        
        if network.realtime_port == 0 {
            return Err(ConfigError::Validation("Realtime port cannot be 0".to_string()));
        }
        
        if network.metrics_port == 0 {
            return Err(ConfigError::Validation("Metrics port cannot be 0".to_string()));
        }
        
        // Check for port conflicts
        let ports = vec![network.api_port, network.realtime_port, network.metrics_port];
        let unique_ports: HashSet<_> = ports.iter().collect();
        if unique_ports.len() != ports.len() {
            return Err(ConfigError::Validation("Port numbers must be unique".to_string()));
        }
        
        if network.max_connections == 0 {
            return Err(ConfigError::Validation("Max connections must be greater than 0".to_string()));
        }
        
        if network.connection_timeout_secs == 0 {
            return Err(ConfigError::Validation("Connection timeout must be greater than 0".to_string()));
        }
        
        Ok(())
    }
    
    fn validate_services(services: &crate::config::ServicesConfig) -> Result<()> {
        // Validate service mesh config
        if services.service_mesh.enabled {
            let valid_lb_types = ["round_robin", "least_request", "random", "ring_hash"];
            if !valid_lb_types.contains(&services.service_mesh.load_balancer_type.as_str()) {
                return Err(ConfigError::Validation(
                    format!("Invalid load balancer type: {}. Must be one of: {:?}", 
                        services.service_mesh.load_balancer_type, valid_lb_types)
                ));
            }
        }
        
        // Validate service discovery
        let valid_providers = ["consul", "etcd", "kubernetes", "static"];
        if !valid_providers.contains(&services.service_discovery.provider.as_str()) {
            return Err(ConfigError::Validation(
                format!("Invalid service discovery provider: {}. Must be one of: {:?}", 
                    services.service_discovery.provider, valid_providers)
            ));
        }
        
        if services.service_discovery.health_check_interval_secs == 0 {
            return Err(ConfigError::Validation("Health check interval cannot be 0".to_string()));
        }
        
        // Validate internal services config
        if services.internal_services.default_timeout_ms == 0 {
            return Err(ConfigError::Validation("Default timeout cannot be 0".to_string()));
        }
        
        Ok(())
    }
            if endpoint.circuit_breaker_threshold < 0.0 || endpoint.circuit_breaker_threshold > 1.0 {
                return Err(ConfigError::Validation(
                    format!("{} circuit breaker threshold must be between 0.0 and 1.0", name)
                ));
            }
        }
        
        Ok(())
    }
    
    fn validate_ai(ai: &crate::config::AIConfig) -> Result<()> {
        // Validate LLM config
        if ai.llm_orchestra.max_tokens == 0 {
            return Err(ConfigError::Validation("LLM max tokens must be greater than 0".to_string()));
        }
        
        if ai.llm_orchestra.temperature < 0.0 || ai.llm_orchestra.temperature > 2.0 {
            return Err(ConfigError::Validation("LLM temperature must be between 0.0 and 2.0".to_string()));
        }
        
        if ai.llm_orchestra.top_p < 0.0 || ai.llm_orchestra.top_p > 1.0 {
            return Err(ConfigError::Validation("LLM top_p must be between 0.0 and 1.0".to_string()));
        }
        
        // Validate procedural generation
        if ai.procedural_generation.creature_diversity < 0.0 || ai.procedural_generation.creature_diversity > 1.0 {
            return Err(ConfigError::Validation("Creature diversity must be between 0.0 and 1.0".to_string()));
        }
        
        if ai.procedural_generation.dungeon_complexity < 0.0 || ai.procedural_generation.dungeon_complexity > 1.0 {
            return Err(ConfigError::Validation("Dungeon complexity must be between 0.0 and 1.0".to_string()));
        }
        
        // Validate behavior AI
        if ai.behavior_ai.npc_update_rate_ms == 0 {
            return Err(ConfigError::Validation("NPC update rate cannot be 0".to_string()));
        }
        
        if ai.behavior_ai.relationship_depth == 0 {
            return Err(ConfigError::Validation("Relationship depth must be greater than 0".to_string()));
        }
        
        Ok(())
    }
    
    fn validate_database(database: &crate::config::DatabaseConfig) -> Result<()> {
        if database.postgres.url.is_empty() {
            return Err(ConfigError::Validation("PostgreSQL URL cannot be empty".to_string()));
        }
        
        if database.postgres.max_connections == 0 {
            return Err(ConfigError::Validation("PostgreSQL max connections must be greater than 0".to_string()));
        }
        
        if database.timescale.url.is_empty() {
            return Err(ConfigError::Validation("TimescaleDB URL cannot be empty".to_string()));
        }
        
        if database.qdrant.url.is_empty() {
            return Err(ConfigError::Validation("Qdrant URL cannot be empty".to_string()));
        }
        
        if database.qdrant.vector_size == 0 {
            return Err(ConfigError::Validation("Qdrant vector size must be greater than 0".to_string()));
        }
        
        let valid_metrics = ["cosine", "euclidean", "dot"];
        if !valid_metrics.contains(&database.qdrant.distance_metric.as_str()) {
            return Err(ConfigError::Validation(
                format!("Invalid distance metric: {}. Must be one of: {:?}", 
                    database.qdrant.distance_metric, valid_metrics)
            ));
        }
        
        Ok(())
    }
    
    fn validate_cache(cache: &crate::config::CacheConfig) -> Result<()> {
        if cache.redis.url.is_empty() {
            return Err(ConfigError::Validation("Redis URL cannot be empty".to_string()));
        }
        
        if cache.redis.pool_size == 0 {
            return Err(ConfigError::Validation("Redis pool size must be greater than 0".to_string()));
        }
        
        if cache.in_memory.max_size_mb == 0 {
            return Err(ConfigError::Validation("In-memory cache size must be greater than 0".to_string()));
        }
        
        let valid_policies = ["lru", "lfu", "arc"];
        if !valid_policies.contains(&cache.in_memory.eviction_policy.as_str()) {
            return Err(ConfigError::Validation(
                format!("Invalid eviction policy: {}. Must be one of: {:?}", 
                    cache.in_memory.eviction_policy, valid_policies)
            ));
        }
        
        Ok(())
    }
    
    fn validate_security(security: &crate::config::SecurityConfig) -> Result<()> {
        if security.jwt_secret.is_empty() {
            return Err(ConfigError::Validation("JWT secret cannot be empty".to_string()));
        }
        
        if security.jwt_secret.len() < 32 {
            return Err(ConfigError::Validation("JWT secret must be at least 32 characters".to_string()));
        }
        
        if security.jwt_expiration_hours == 0 {
            return Err(ConfigError::Validation("JWT expiration must be greater than 0".to_string()));
        }
        
        if security.rate_limiting.enabled && security.rate_limiting.requests_per_minute == 0 {
            return Err(ConfigError::Validation("Rate limit requests per minute must be greater than 0".to_string()));
        }
        
        Ok(())
    }
    
    fn validate_performance(performance: &crate::config::PerformanceConfig) -> Result<()> {
        if performance.worker_threads == 0 {
            return Err(ConfigError::Validation("Worker threads must be greater than 0".to_string()));
        }
        
        if performance.async_runtime_threads == 0 {
            return Err(ConfigError::Validation("Async runtime threads must be greater than 0".to_string()));
        }
        
        if performance.connection_pool_size == 0 {
            return Err(ConfigError::Validation("Connection pool size must be greater than 0".to_string()));
        }
        
        if performance.batch_processing_size == 0 {
            return Err(ConfigError::Validation("Batch processing size must be greater than 0".to_string()));
        }
        
        Ok(())
    }
    
    fn validate_monitoring(monitoring: &crate::config::MonitoringConfig) -> Result<()> {
        if monitoring.metrics_enabled && monitoring.metrics_port == 0 {
            return Err(ConfigError::Validation("Metrics port cannot be 0 when metrics are enabled".to_string()));
        }
        
        if monitoring.log_sampling_rate < 0.0 || monitoring.log_sampling_rate > 1.0 {
            return Err(ConfigError::Validation("Log sampling rate must be between 0.0 and 1.0".to_string()));
        }
        
        Ok(())
    }
    
    fn validate_game(game: &crate::config::GameConfig) -> Result<()> {
        // Validate world settings
        if game.world_settings.default_region_size == 0 {
            return Err(ConfigError::Validation("Default region size must be greater than 0".to_string()));
        }
        
        if game.world_settings.max_players_per_region == 0 {
            return Err(ConfigError::Validation("Max players per region must be greater than 0".to_string()));
        }
        
        if game.world_settings.weather_change_probability < 0.0 || 
           game.world_settings.weather_change_probability > 1.0 {
            return Err(ConfigError::Validation("Weather change probability must be between 0.0 and 1.0".to_string()));
        }
        
        // Validate harmony settings
        if game.harmony_settings.max_attunement_level == 0 {
            return Err(ConfigError::Validation("Max attunement level must be greater than 0".to_string()));
        }
        
        if game.harmony_settings.collaboration_multiplier < 1.0 {
            return Err(ConfigError::Validation("Collaboration multiplier must be at least 1.0".to_string()));
        }
        
        // Validate echo settings
        if game.echo_settings.max_bond_level == 0 {
            return Err(ConfigError::Validation("Max bond level must be greater than 0".to_string()));
        }
        
        // Validate event settings
        if game.event_settings.max_concurrent_events == 0 {
            return Err(ConfigError::Validation("Max concurrent events must be greater than 0".to_string()));
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_validate_valid_config() {
        let config = FinalverseConfig::default();
        assert!(ConfigValidator::validate(&config).is_ok());
    }
    
    #[test]
    fn test_validate_invalid_port() {
        let mut config = FinalverseConfig::default();
        config.network.port = 0;
        assert!(ConfigValidator::validate(&config).is_err());
    }
    
    #[test]
    fn test_validate_invalid_jwt_secret() {
        let mut config = FinalverseConfig::default();
        config.security.jwt_secret = "short".to_string();
        assert!(ConfigValidator::validate(&config).is_err());
    }
}