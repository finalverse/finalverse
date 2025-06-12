// crates/fv-events/src/nats.rs
use futures_util::StreamExt;
use async_nats::{Client, Subscriber};
use tokio::sync::RwLock;
use std::sync::Arc;
use std::collections::HashMap;
use uuid::Uuid;

use crate::event_bus::GameEventBus;

pub struct NatsEventBus {
    client: Arc<RwLock<Client>>,
    subscriptions: Arc<RwLock<HashMap<String, Subscriber>>>,
}

impl NatsEventBus {
    pub async fn new(nats_url: &str) -> anyhow::Result<Self> {
        let client = async_nats::connect(nats_url).await?;
        Ok(Self {
            client: Arc::new(RwLock::new(client)),
            subscriptions: Arc::new(RwLock::new(HashMap::new())),
        })
    }
}

#[async_trait::async_trait]
impl GameEventBus for NatsEventBus {
    async fn publish_raw(&self, topic: &str, payload: Vec<u8>) -> anyhow::Result<()> {
        self.client
            .read()
            .await
            .publish(topic.to_string(), payload.into())
            .await?;
        Ok(())
    }

    async fn subscribe_raw<F>(&self, topic: &str, handler: F) -> anyhow::Result<String>
    where
        F: Fn(Vec<u8>) + Send + Sync + 'static,
    {
        let subscriber = self.client.read().await.subscribe(topic.to_string()).await?;
        let subscription_id = Uuid::new_v4().to_string();
        
        let sub_id_clone = subscription_id.clone();
        let subscriptions = self.subscriptions.clone();
        
        // Store the subscriber
        subscriptions.write().await.insert(sub_id_clone.clone(), subscriber);
        
        // Spawn handler task
        tokio::spawn(async move {
            let mut sub = subscriptions.write().await.remove(&sub_id_clone).unwrap();
            while let Some(msg) = sub.next().await {
                handler(msg.payload.to_vec());
            }
        });
        
        Ok(subscription_id)
    }
    
    async fn unsubscribe(&self, subscription_id: &str) -> anyhow::Result<()> {
        self.subscriptions.write().await.remove(subscription_id);
        Ok(())
    }
}
