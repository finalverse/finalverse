// services/service-registry/src/lib.rs
// Service discovery and registration for Finalverse

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use tokio::time::interval;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceInstance {
    pub id: String,
    pub name: String,
    pub host: String,
    pub port: u16,
    pub health_check_url: String,
    pub metadata: HashMap<String, String>,
    #[serde(skip)]
    pub last_heartbeat: Instant,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceRegistration {
    pub name: String,
    pub host: String,
    pub port: u16,
    pub health_check_path: String,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct ServiceRegistry {
    services: Arc<RwLock<HashMap<String, Vec<ServiceInstance>>>>,
    health_check_interval: Duration,
    heartbeat_timeout: Duration,
}

impl Default for ServiceRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl ServiceRegistry {
    pub fn new() -> Self {
        Self {
            services: Arc::new(RwLock::new(HashMap::new())),
            health_check_interval: Duration::from_secs(10),
            heartbeat_timeout: Duration::from_secs(30),
        }
    }
    
    pub async fn register(&self, registration: ServiceRegistration) -> String {
        let id = format!("{}-{}", registration.name, uuid::Uuid::new_v4());
        
        let instance = ServiceInstance {
            id: id.clone(),
            name: registration.name.clone(),
            host: registration.host,
            port: registration.port,
            health_check_url: format!(
                "http://{}:{}{}",
                registration.host,
                registration.port,
                registration.health_check_path
            ),
            metadata: registration.metadata,
            last_heartbeat: Instant::now(),
        };
        
        let mut services = self.services.write().await;
        services
            .entry(registration.name)
            .or_insert_with(Vec::new)
            .push(instance);
        
        id
    }
    
    pub async fn deregister(&self, service_id: &str) {
        let mut services = self.services.write().await;
        
        for instances in services.values_mut() {
            instances.retain(|instance| instance.id != service_id);
        }
        
        // Remove empty entries
        services.retain(|_, instances| !instances.is_empty());
    }
    
    pub async fn heartbeat(&self, service_id: &str) -> bool {
        let mut services = self.services.write().await;
        
        for instances in services.values_mut() {
            for instance in instances.iter_mut() {
                if instance.id == service_id {
                    instance.last_heartbeat = Instant::now();
                    return true;
                }
            }
        }
        
        false
    }
    
    pub async fn discover(&self, service_name: &str) -> Option<ServiceInstance> {
        let services = self.services.read().await;
        
        services.get(service_name)
            .and_then(|instances| {
                // Find healthy instances
                let now = Instant::now();
                instances
                    .iter()
                    .filter(|instance| {
                        now.duration_since(instance.last_heartbeat) < self.heartbeat_timeout
                    })
                    .min_by_key(|_| rand::random::<u8>()) // Random load balancing
                    .cloned()
            })
    }
    
    pub async fn discover_all(&self, service_name: &str) -> Vec<ServiceInstance> {
        let services = self.services.read().await;
        let now = Instant::now();
        
        services.get(service_name)
            .map(|instances| {
                instances
                    .iter()
                    .filter(|instance| {
                        now.duration_since(instance.last_heartbeat) < self.heartbeat_timeout
                    })
                    .cloned()
                    .collect()
            })
            .unwrap_or_default()
    }
    
    pub async fn list_services(&self) -> HashMap<String, Vec<ServiceInstance>> {
        let services = self.services.read().await;
        let now = Instant::now();
        
        services
            .iter()
            .map(|(name, instances)| {
                let healthy_instances: Vec<ServiceInstance> = instances
                    .iter()
                    .filter(|instance| {
                        now.duration_since(instance.last_heartbeat) < self.heartbeat_timeout
                    })
                    .cloned()
                    .collect();
                (name.clone(), healthy_instances)
            })
            .filter(|(_, instances)| !instances.is_empty())
            .collect()
    }
    
    pub async fn cleanup_stale_services(&self) {
        let mut services = self.services.write().await;
        let now = Instant::now();
        
        for instances in services.values_mut() {
            instances.retain(|instance| {
                now.duration_since(instance.last_heartbeat) < self.heartbeat_timeout
            });
        }
        
        services.retain(|_, instances| !instances.is_empty());
    }
    
    pub fn start_cleanup_task(&self) {
        let registry = self.clone();
        tokio::spawn(async move {
            let mut ticker = interval(Duration::from_secs(30));
            
            loop {
                ticker.tick().await;
                registry.cleanup_stale_services().await;
            }
        });
    }
}

// Client for services to interact with the registry
pub struct RegistryClient {
    registry_url: String,
    service_id: Option<String>,
    client: reqwest::Client,
}

impl RegistryClient {
    pub fn new(registry_url: impl Into<String>) -> Self {
        Self {
            registry_url: registry_url.into(),
            service_id: None,
            client: reqwest::Client::new(),
        }
    }
    
    pub async fn register(&mut self, registration: ServiceRegistration) -> anyhow::Result<()> {
        let response = self.client
            .post(&format!("{}/register", self.registry_url))
            .json(&registration)
            .send()
            .await?;
        
        if response.status().is_success() {
            let id: String = response.json().await?;
            self.service_id = Some(id);
            Ok(())
        } else {
            Err(anyhow::anyhow!("Registration failed: {}", response.status()))
        }
    }
    
    pub async fn deregister(&self) -> anyhow::Result<()> {
        if let Some(id) = &self.service_id {
            self.client
                .delete(&format!("{}/services/{}", self.registry_url, id))
                .send()
                .await?;
        }
        Ok(())
    }
    
    pub async fn heartbeat(&self) -> anyhow::Result<()> {
        if let Some(id) = &self.service_id {
            self.client
                .put(&format!("{}/services/{}/heartbeat", self.registry_url, id))
                .send()
                .await?;
        }
        Ok(())
    }
    
    pub fn start_heartbeat_task(&self) {
        if let Some(id) = &self.service_id {
            let client = self.client.clone();
            let registry_url = self.registry_url.clone();
            let service_id = id.clone();
            
            tokio::spawn(async move {
                let mut ticker = interval(Duration::from_secs(10));
                
                loop {
                    ticker.tick().await;
                    let _ = client
                        .put(&format!("{}/services/{}/heartbeat", registry_url, service_id))
                        .send()
                        .await;
                }
            });
        }
    }
    
    pub async fn discover(&self, service_name: &str) -> anyhow::Result<Option<ServiceInstance>> {
        let response = self.client
            .get(&format!("{}/discover/{}", self.registry_url, service_name))
            .send()
            .await?;
        
        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            Ok(None)
        }
    }
}

// For local development without external registry
#[derive(Clone)]
pub struct LocalServiceRegistry {
    services: Arc<RwLock<HashMap<String, String>>>,
}

impl Default for LocalServiceRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl LocalServiceRegistry {
    pub fn new() -> Self {
        let mut services = HashMap::new();
        
        // Pre-populate with known services for local development
        services.insert("song-engine".to_string(), "http://localhost:3001".to_string());
        services.insert("world-engine".to_string(), "http://localhost:3002".to_string());
        services.insert("echo-engine".to_string(), "http://localhost:3003".to_string());
        services.insert("ai-orchestra".to_string(), "http://localhost:3004".to_string());
        services.insert("story-engine".to_string(), "http://localhost:3005".to_string());
        services.insert("harmony-service".to_string(), "http://localhost:3006".to_string());
        services.insert("asset-service".to_string(), "http://localhost:3007".to_string());
        services.insert("community-service".to_string(), "http://localhost:3008".to_string());
        services.insert("silence-service".to_string(), "http://localhost:3009".to_string());
        services.insert("procedural-gen".to_string(), "http://localhost:3010".to_string());
        services.insert("behavior-ai".to_string(), "http://localhost:3011".to_string());
        
        Self {
            services: Arc::new(RwLock::new(services)),
        }
    }
    
    pub async fn get_service_url(&self, service_name: &str) -> Option<String> {
        let services = self.services.read().await;
        services.get(service_name).cloned()
    }
    
    pub async fn register_service(&self, name: String, url: String) {
        let mut services = self.services.write().await;
        services.insert(name, url);
    }
}