// finalverse-config/src/config.rs

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::SocketAddr;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinalverseConfig {
    pub general: GeneralConfig,
    pub network: NetworkConfig,
    pub services: ServicesConfig,
    pub ai: AIConfig,
    pub database: DatabaseConfig,
    pub cache: CacheConfig,
    pub security: SecurityConfig,
    pub performance: PerformanceConfig,
    pub monitoring: MonitoringConfig,
    pub game: GameConfig,
    pub grpc_services: GrpcServiceRegistry,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralConfig {
    pub server_name: String,
    pub version: String,
    pub environment: Environment,
    pub debug_mode: bool,
    pub log_level: String,
    pub log_format: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Environment {
    Development,
    Staging,
    Production,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    pub host: String,
    pub api_port: u16,           // Single API gateway port (8080)
    pub realtime_port: u16,      // Single WebSocket/WebTransport port (8081)
    pub metrics_port: u16,       // Metrics/admin port (9090)
    pub enable_tls: bool,        // Enable TLS on standard ports (443)
    pub public_api_url: String,
    pub public_realtime_url: String,
    pub cors_origins: Vec<String>,
    pub max_connections: usize,
    pub connection_timeout_secs: u64,
    pub enable_http3: bool,      // Enable QUIC/HTTP3
    pub enable_webtransport: bool, // Enable WebTransport protocol
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServicesConfig {
    pub service_mesh: ServiceMeshConfig,
    pub service_discovery: ServiceDiscoveryConfig,
    pub internal_services: InternalServicesConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceMeshConfig {
    pub enabled: bool,
    pub auto_mtls: bool,
    pub auto_retry: bool,
    pub circuit_breaker_enabled: bool,
    pub load_balancer_type: String, // "round_robin", "least_request", "random"
    pub tracing_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceDiscoveryConfig {
    pub provider: String, // "consul", "etcd", "kubernetes"
    pub health_check_interval_secs: u64,
    pub deregister_critical_after_secs: u64,
    pub enable_auto_registration: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InternalServicesConfig {
    pub auto_discover: bool,
    pub namespace: String,
    pub default_timeout_ms: u64,
    pub default_retries: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceEndpoint {
    pub enabled: bool,
    pub url: String,
    pub timeout_ms: u64,
    pub max_retries: u32,
    pub circuit_breaker_threshold: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrpcServiceRegistry {
    pub services: HashMap<String, SocketAddr>,
}

impl GrpcServiceRegistry {
    pub fn new(map: HashMap<String, SocketAddr>) -> Self {
        Self { services: map }
    }
}

impl Default for GrpcServiceRegistry {
    fn default() -> Self {
        let mut map = HashMap::new();
        map.insert("song-engine".to_string(), "127.0.0.1:50051".parse().unwrap());
        map.insert("story-engine".to_string(), "127.0.0.1:50052".parse().unwrap());
        Self { services: map }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIConfig {
    pub llm_orchestra: LLMConfig,
    pub procedural_generation: ProceduralGenConfig,
    pub behavior_ai: BehaviorAIConfig,
    pub vision_ai: VisionAIConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMConfig {
    pub models: HashMap<String, LLMModel>,
    pub default_model: String,
    pub max_tokens: usize,
    pub temperature: f32,
    pub top_p: f32,
    pub context_window: usize,
    pub cache_responses: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMModel {
    pub provider: String,
    pub model_name: String,
    pub api_key: String,
    pub endpoint_url: Option<String>,
    pub max_requests_per_minute: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProceduralGenConfig {
    pub terrain_seed: u64,
    pub creature_diversity: f32,
    pub item_rarity_distribution: Vec<f32>,
    pub dungeon_complexity: f32,
    pub ai_enhancement_level: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehaviorAIConfig {
    pub npc_update_rate_ms: u64,
    pub creature_ai_complexity: String,
    pub emotion_modeling: bool,
    pub memory_persistence: bool,
    pub relationship_depth: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisionAIConfig {
    pub enabled: bool,
    pub model_path: String,
    pub inference_device: String,
    pub batch_size: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub postgres: PostgresConfig,
    pub timescale: TimescaleConfig,
    pub qdrant: QdrantConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostgresConfig {
    pub url: String,
    pub max_connections: u32,
    pub connection_timeout_secs: u64,
    pub ssl_mode: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimescaleConfig {
    pub url: String,
    pub chunk_time_interval: String,
    pub compression_after: String,
    pub retention_policy: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QdrantConfig {
    pub url: String,
    pub collection_name: String,
    pub vector_size: usize,
    pub distance_metric: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    pub redis: RedisConfig,
    pub in_memory: InMemoryCacheConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedisConfig {
    pub url: String,
    pub cluster_mode: bool,
    pub password: Option<String>,
    pub db: u8,
    pub pool_size: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InMemoryCacheConfig {
    pub max_size_mb: usize,
    pub ttl_seconds: u64,
    pub eviction_policy: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    pub jwt_secret: String,
    pub jwt_expiration_hours: u64,
    pub rate_limiting: RateLimitConfig,
    pub encryption: EncryptionConfig,
    pub allowed_origins: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    pub enabled: bool,
    pub requests_per_minute: u32,
    pub burst_size: u32,
    pub ip_whitelist: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionConfig {
    pub algorithm: String,
    pub key_rotation_days: u32,
    pub data_at_rest: bool,
    pub data_in_transit: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfig {
    pub worker_threads: usize,
    pub async_runtime_threads: usize,
    pub connection_pool_size: u32,
    pub batch_processing_size: usize,
    pub compression_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringConfig {
    pub metrics_enabled: bool,
    pub metrics_port: u16,
    pub prometheus_endpoint: String,
    pub tracing_enabled: bool,
    pub tracing_endpoint: String,
    pub log_sampling_rate: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameConfig {
    pub world_settings: WorldSettings,
    pub harmony_settings: HarmonySettings,
    pub echo_settings: EchoSettings,
    pub event_settings: EventSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldSettings {
    pub default_region_size: u32,
    pub max_players_per_region: u32,
    pub day_night_cycle_minutes: u32,
    pub weather_change_probability: f32,
    pub ecosystem_update_rate_seconds: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HarmonySettings {
    pub base_resonance_gain: f32,
    pub collaboration_multiplier: f32,
    pub decay_rate_per_hour: f32,
    pub max_attunement_level: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EchoSettings {
    pub bond_gain_rate: f32,
    pub teaching_cooldown_minutes: u32,
    pub max_bond_level: u32,
    pub echo_spawn_locations: HashMap<String, Vec<f32>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventSettings {
    pub world_event_frequency_hours: u32,
    pub silence_spread_rate: f32,
    pub player_event_cooldown_minutes: u32,
    pub max_concurrent_events: u32,
}

impl Default for FinalverseConfig {
    fn default() -> Self {
        Self {
            general: GeneralConfig {
                server_name: "Finalverse".to_string(),
                version: "0.1.1".to_string(),
                environment: Environment::Development,
                debug_mode: false,
                log_level: "info".to_string(),
                log_format: "json".to_string(),
            },
            network: NetworkConfig {
                host: "0.0.0.0".to_string(),
                api_port: 8080,
                realtime_port: 8081,
                metrics_port: 9090,
                enable_tls: false,
                public_api_url: "http://localhost:8080".to_string(),
                public_realtime_url: "ws://localhost:8081".to_string(),
                cors_origins: vec!["*".to_string()],
                max_connections: 10000,
                connection_timeout_secs: 30,
                enable_http3: false,
                enable_webtransport: false,
            },
            services: ServicesConfig {
                service_mesh: ServiceMeshConfig {
                    enabled: true,
                    auto_mtls: true,
                    auto_retry: true,
                    circuit_breaker_enabled: true,
                    load_balancer_type: "least_request".to_string(),
                    tracing_enabled: true,
                },
                service_discovery: ServiceDiscoveryConfig {
                    provider: "consul".to_string(),
                    health_check_interval_secs: 5,
                    deregister_critical_after_secs: 30,
                    enable_auto_registration: true,
                },
                internal_services: InternalServicesConfig {
                    auto_discover: true,
                    namespace: "finalverse".to_string(),
                    default_timeout_ms: 5000,
                    default_retries: 3,
                },
            },
            ai: AIConfig {
                llm_orchestra: LLMConfig::default(),
                procedural_generation: ProceduralGenConfig::default(),
                behavior_ai: BehaviorAIConfig::default(),
                vision_ai: VisionAIConfig::default(),
            },
            database: DatabaseConfig {
                postgres: PostgresConfig::default(),
                timescale: TimescaleConfig::default(),
                qdrant: QdrantConfig::default(),
            },
            cache: CacheConfig {
                redis: RedisConfig::default(),
                in_memory: InMemoryCacheConfig::default(),
            },
            security: SecurityConfig::default(),
            performance: PerformanceConfig::default(),
            monitoring: MonitoringConfig::default(),
            game: GameConfig::default(),
            grpc_services: GrpcServiceRegistry::default(),
        }
    }
}

// Implement defaults for all sub-configurations
impl Default for ServiceEndpoint {
    fn default() -> Self {
        Self {
            enabled: true,
            url: "http://localhost:9000".to_string(),
            timeout_ms: 5000,
            max_retries: 3,
            circuit_breaker_threshold: 0.5,
        }
    }
}

impl Default for LLMConfig {
    fn default() -> Self {
        Self {
            models: HashMap::new(),
            default_model: "gpt-4".to_string(),
            max_tokens: 2048,
            temperature: 0.7,
            top_p: 0.9,
            context_window: 4096,
            cache_responses: true,
        }
    }
}

impl Default for ProceduralGenConfig {
    fn default() -> Self {
        Self {
            terrain_seed: 42,
            creature_diversity: 0.7,
            item_rarity_distribution: vec![0.6, 0.25, 0.10, 0.04, 0.01],
            dungeon_complexity: 0.5,
            ai_enhancement_level: 0.8,
        }
    }
}

impl Default for BehaviorAIConfig {
    fn default() -> Self {
        Self {
            npc_update_rate_ms: 100,
            creature_ai_complexity: "medium".to_string(),
            emotion_modeling: true,
            memory_persistence: true,
            relationship_depth: 5,
        }
    }
}

impl Default for VisionAIConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            model_path: "./models/vision_model.onnx".to_string(),
            inference_device: "cpu".to_string(),
            batch_size: 8,
        }
    }
}

impl Default for PostgresConfig {
    fn default() -> Self {
        Self {
            url: "postgresql://finalverse:password@localhost/finalverse".to_string(),
            max_connections: 50,
            connection_timeout_secs: 30,
            ssl_mode: "prefer".to_string(),
        }
    }
}

impl Default for TimescaleConfig {
    fn default() -> Self {
        Self {
            url: "postgresql://finalverse:password@localhost/finalverse_timescale".to_string(),
            chunk_time_interval: "1 day".to_string(),
            compression_after: "7 days".to_string(),
            retention_policy: "90 days".to_string(),
        }
    }
}

impl Default for QdrantConfig {
    fn default() -> Self {
        Self {
            url: "http://localhost:6333".to_string(),
            collection_name: "finalverse_embeddings".to_string(),
            vector_size: 1536,
            distance_metric: "cosine".to_string(),
        }
    }
}

impl Default for RedisConfig {
    fn default() -> Self {
        Self {
            url: "redis://localhost:6379".to_string(),
            cluster_mode: false,
            password: None,
            db: 0,
            pool_size: 50,
        }
    }
}

impl Default for InMemoryCacheConfig {
    fn default() -> Self {
        Self {
            max_size_mb: 512,
            ttl_seconds: 3600,
            eviction_policy: "lru".to_string(),
        }
    }
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            jwt_secret: "change-this-secret-in-production-minimum-32-chars".to_string(),
            jwt_expiration_hours: 24,
            rate_limiting: RateLimitConfig::default(),
            encryption: EncryptionConfig::default(),
            allowed_origins: vec!["*".to_string()],
        }
    }
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            requests_per_minute: 60,
            burst_size: 10,
            ip_whitelist: vec![],
        }
    }
}

impl Default for EncryptionConfig {
    fn default() -> Self {
        Self {
            algorithm: "AES-256-GCM".to_string(),
            key_rotation_days: 90,
            data_at_rest: true,
            data_in_transit: true,
        }
    }
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            worker_threads: num_cpus::get(),
            async_runtime_threads: num_cpus::get() * 2,
            connection_pool_size: 100,
            batch_processing_size: 100,
            compression_enabled: true,
        }
    }
}

impl Default for MonitoringConfig {
    fn default() -> Self {
        Self {
            metrics_enabled: true,
            metrics_port: 9090,
            prometheus_endpoint: "/metrics".to_string(),
            tracing_enabled: true,
            tracing_endpoint: "http://localhost:14268/api/traces".to_string(),
            log_sampling_rate: 1.0,
        }
    }
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            world_settings: WorldSettings::default(),
            harmony_settings: HarmonySettings::default(),
            echo_settings: EchoSettings::default(),
            event_settings: EventSettings::default(),
        }
    }
}

impl Default for WorldSettings {
    fn default() -> Self {
        Self {
            default_region_size: 1024,
            max_players_per_region: 100,
            day_night_cycle_minutes: 60,
            weather_change_probability: 0.1,
            ecosystem_update_rate_seconds: 30,
        }
    }
}

impl Default for HarmonySettings {
    fn default() -> Self {
        Self {
            base_resonance_gain: 1.0,
            collaboration_multiplier: 1.5,
            decay_rate_per_hour: 0.05,
            max_attunement_level: 100,
        }
    }
}

impl Default for EchoSettings {
    fn default() -> Self {
        let mut spawn_locations = HashMap::new();
        spawn_locations.insert("lumi".to_string(), vec![0.0, 100.0, 0.0]);
        spawn_locations.insert("kai".to_string(), vec![100.0, 100.0, 0.0]);
        spawn_locations.insert("terra".to_string(), vec![-100.0, 100.0, 0.0]);
        spawn_locations.insert("ignis".to_string(), vec![0.0, 100.0, 100.0]);
        
        Self {
            bond_gain_rate: 0.1,
            teaching_cooldown_minutes: 30,
            max_bond_level: 50,
            echo_spawn_locations: spawn_locations,
        }
    }
}

impl Default for EventSettings {
    fn default() -> Self {
        Self {
            world_event_frequency_hours: 24,
            silence_spread_rate: 0.01,
            player_event_cooldown_minutes: 60,
            max_concurrent_events: 10,
        }
    }
}