// crates/fv-events/src/local.rs
use tokio::sync::{broadcast, RwLock};
use std::sync::Arc;
use std::collections::HashMap;
use uuid::Uuid;

use crate::event_bus::GameEventBus;

/// Local in-memory event bus for testing and single-node deployments
pub struct LocalEventBus {
    channels: Arc<RwLock<HashMap<String, broadcast::Sender<Vec<u8>>>>>,
    subscriptions: Arc<RwLock<HashMap<String, broadcast::Receiver<Vec<u8>>>>>,
}

impl LocalEventBus {
    pub fn new() -> Self {
        Self {
            channels: Arc::new(RwLock::new(HashMap::new())),
            subscriptions: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl Default for LocalEventBus {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl GameEventBus for LocalEventBus {
    async fn publish_raw(&self, topic: &str, payload: Vec<u8>) -> anyhow::Result<()> {
        let channels = self.channels.read().await;
        
        if let Some(sender) = channels.get(topic) {
            // Ignore send errors (no receivers)
            let _ = sender.send(payload);
        }
        
        Ok(())
    }

    async fn subscribe_raw(
        &self,
        topic: &str,
        handler: Box<dyn Fn(Vec<u8>) + Send + Sync + 'static>,
    ) -> anyhow::Result<String> {
        let subscription_id = Uuid::new_v4().to_string();
        
        // Get or create channel for topic
        let receiver = {
            let mut channels = self.channels.write().await;
            let sender = channels.entry(topic.to_string())
                .or_insert_with(|| {
                    let (tx, _) = broadcast::channel(1000);
                    tx
                });
            sender.subscribe()
        };
        
        // Store receiver
        self.subscriptions.write().await.insert(subscription_id.clone(), receiver);
        
        // Spawn handler task
        let sub_id_clone = subscription_id.clone();
        let subscriptions = self.subscriptions.clone();
        tokio::spawn(async move {
            if let Some(mut receiver) = subscriptions.write().await.remove(&sub_id_clone) {
                let handler = handler;
                while let Ok(payload) = receiver.recv().await {
                    handler(payload);
                }
            }
        });
        
        Ok(subscription_id)
    }
    
    async fn unsubscribe(&self, subscription_id: &str) -> anyhow::Result<()> {
        self.subscriptions.write().await.remove(subscription_id);
        Ok(())
    }
}