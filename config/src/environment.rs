// finalverse-config/src/environment.rs

use crate::{FinalverseConfig, ConfigError, Result};
use std::env;

/// Apply environment variable overrides to the configuration
pub fn apply_env_overrides(config: &mut FinalverseConfig) -> Result<()> {
    // General settings
    if let Ok(server_name) = env::var("FINALVERSE_SERVER_NAME") {
        config.general.server_name = server_name;
    }
    
    if let Ok(debug) = env::var("FINALVERSE_DEBUG") {
        config.general.debug_mode = debug.parse().unwrap_or(false);
    }
    
    if let Ok(log_level) = env::var("FINALVERSE_LOG_LEVEL") {
        config.general.log_level = log_level;
    }
    
    // Network settings
    if let Ok(host) = env::var("FINALVERSE_HOST") {
        config.network.host = host;
    }
    
    if let Ok(port) = env::var("FINALVERSE_API_PORT") {
        config.network.api_port = port.parse()
            .map_err(|_| ConfigError::Environment("Invalid FINALVERSE_API_PORT".to_string()))?;
    }

    if let Ok(rt_port) = env::var("FINALVERSE_REALTIME_PORT") {
        config.network.realtime_port = rt_port.parse()
            .map_err(|_| ConfigError::Environment("Invalid FINALVERSE_REALTIME_PORT".to_string()))?;
    }

    if let Ok(metrics_port) = env::var("FINALVERSE_METRICS_PORT") {
        config.network.metrics_port = metrics_port.parse()
            .map_err(|_| ConfigError::Environment("Invalid FINALVERSE_METRICS_PORT".to_string()))?;
    }
    
    // Database settings
    if let Ok(db_url) = env::var("FINALVERSE_DATABASE_URL") {
        config.database.postgres.url = db_url.clone();
        config.database.timescale.url = db_url;
    }
    
    if let Ok(redis_url) = env::var("FINALVERSE_REDIS_URL") {
        config.cache.redis.url = redis_url;
    }
    
    if let Ok(qdrant_url) = env::var("FINALVERSE_QDRANT_URL") {
        config.database.qdrant.url = qdrant_url;
    }
    
    // Security settings
    if let Ok(jwt_secret) = env::var("FINALVERSE_JWT_SECRET") {
        config.security.jwt_secret = jwt_secret;
    }
    
    // AI settings
    apply_ai_env_overrides(&mut config.ai)?;
    
    // Performance settings
    if let Ok(workers) = env::var("FINALVERSE_WORKER_THREADS") {
        config.performance.worker_threads = workers.parse()
            .map_err(|_| ConfigError::Environment("Invalid FINALVERSE_WORKER_THREADS".to_string()))?;
    }
    
    Ok(())
}

fn apply_ai_env_overrides(ai_config: &mut crate::config::AIConfig) -> Result<()> {
    // LLM settings
    if let Ok(api_key) = env::var("OPENAI_API_KEY") {
        ai_config.llm_orchestra.models.insert(
            "openai".to_string(),
            crate::config::LLMModel {
                provider: "openai".to_string(),
                model_name: "gpt-4".to_string(),
                api_key,
                endpoint_url: None,
                max_requests_per_minute: 60,
            },
        );
    }
    
    if let Ok(anthropic_key) = env::var("ANTHROPIC_API_KEY") {
        ai_config.llm_orchestra.models.insert(
            "anthropic".to_string(),
            crate::config::LLMModel {
                provider: "anthropic".to_string(),
                model_name: "claude-3-opus-20240229".to_string(),
                api_key: anthropic_key,
                endpoint_url: None,
                max_requests_per_minute: 50,
            },
        );
    }
    
    if let Ok(default_model) = env::var("FINALVERSE_DEFAULT_LLM") {
        ai_config.llm_orchestra.default_model = default_model;
    }
    
    Ok(())
}

/// Get all environment variables with FINALVERSE_ prefix
pub fn get_finalverse_env_vars() -> Vec<(String, String)> {
    env::vars()
        .filter(|(k, _)| k.starts_with("FINALVERSE_"))
        .collect()
}

/// Create a sample .env file with all available environment variables
pub fn generate_env_template() -> String {
    r#"# Finalverse Environment Configuration

# General Settings
FINALVERSE_SERVER_NAME=Finalverse
FINALVERSE_DEBUG=false
FINALVERSE_LOG_LEVEL=info

# Network Settings
FINALVERSE_HOST=0.0.0.0
FINALVERSE_API_PORT=8080
FINALVERSE_REALTIME_PORT=8081
FINALVERSE_METRICS_PORT=9090

# Database Settings
FINALVERSE_DATABASE_URL=postgresql://finalverse:password@localhost/finalverse
FINALVERSE_REDIS_URL=redis://localhost:6379
FINALVERSE_QDRANT_URL=http://localhost:6333

# Security Settings
FINALVERSE_JWT_SECRET=your-secret-key-here

# AI Settings
OPENAI_API_KEY=your-openai-api-key
ANTHROPIC_API_KEY=your-anthropic-api-key
FINALVERSE_DEFAULT_LLM=openai

# Performance Settings
FINALVERSE_WORKER_THREADS=8
"#
        .to_string()
}