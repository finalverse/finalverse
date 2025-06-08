// libs/protocol/src/event_bus.rs - Simplified version

use crate::*;
use finalverse_common::*;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use tracing::{info, warn};

pub struct InMemoryEventBus {
    subscribers: Arc<RwLock<HashMap<String, Vec<mpsc::Sender<FinalverseEvent>>>>>,
}

impl InMemoryEventBus {
    pub fn new() -> Self {
        Self {
            subscribers: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait::async_trait]
impl EventBus for InMemoryEventBus {
    async fn publish(&self, event: FinalverseEvent) -> Result<(), FinalverseError> {
        let subscribers = self.subscribers.read().await;
        
        info!("Publishing event: {:?}", event);
        
        // Send to all subscribers
        for (service_name, senders) in subscribers.iter() {
            for sender in senders {
                if let Err(e) = sender.send(event.clone()).await {
                    warn!("Failed to send event to {}: {}", service_name, e);
                }
            }
        }
        
        Ok(())
    }
    
    async fn subscribe(&self, service_name: &str) -> Result<mpsc::Receiver<FinalverseEvent>, FinalverseError> {
        let (tx, rx) = mpsc::channel(100);
        
        let mut subscribers = self.subscribers.write().await;
        subscribers
            .entry(service_name.to_string())
            .or_insert_with(Vec::new)
            .push(tx);
        
        info!("{} subscribed to event bus", service_name);
        
        Ok(rx)
    }
}

// Simplified Redis event bus that uses basic async connection
pub struct RedisEventBus {
    redis_url: String,
    local_subscribers: Arc<RwLock<HashMap<String, Vec<mpsc::Sender<FinalverseEvent>>>>>,
}

impl RedisEventBus {
    pub fn new(redis_url: &str) -> Result<Self, FinalverseError> {
        Ok(Self {
            redis_url: redis_url.to_string(),
            local_subscribers: Arc::new(RwLock::new(HashMap::new())),
        })
    }
    
    // For MVP, we'll use a polling approach instead of pub/sub
    pub async fn start_listening(self: Arc<Self>) {
        info!("Redis event bus listening started (polling mode for MVP)");
        // In a production system, this would use Redis pub/sub
        // For MVP, we'll rely on direct service-to-service calls
    }
}

#[async_trait::async_trait]
impl EventBus for RedisEventBus {
    async fn publish(&self, event: FinalverseEvent) -> Result<(), FinalverseError> {
        // For MVP, we'll use in-memory distribution
        let subscribers = self.local_subscribers.read().await;
        
        info!("Publishing event via Redis bus: {:?}", event);
        
        for (service_name, senders) in subscribers.iter() {
            for sender in senders {
                if let Err(e) = sender.send(event.clone()).await {
                    warn!("Failed to send event to {}: {}", service_name, e);
                }
            }
        }
        
        Ok(())
    }
    
    async fn subscribe(&self, service_name: &str) -> Result<mpsc::Receiver<FinalverseEvent>, FinalverseError> {
        let (tx, rx) = mpsc::channel(100);
        
        let mut subscribers = self.local_subscribers.write().await;
        subscribers
            .entry(service_name.to_string())
            .or_insert_with(Vec::new)
            .push(tx);
        
        info!("{} subscribed to Redis event bus", service_name);
        
        Ok(rx)
    }
}