use futures_util::StreamExt;
use async_nats::Client;
use tokio::sync::RwLock;
use std::sync::Arc;

use crate::event_bus::GameEventBus;

pub struct NatsEventBus {
    client: Arc<RwLock<Client>>,
}

impl NatsEventBus {
    pub async fn new(nats_url: &str) -> anyhow::Result<Self> {
        let client = async_nats::connect(nats_url).await?;
        Ok(Self {
            client: Arc::new(RwLock::new(client)),
        })
    }
}

#[async_trait::async_trait]
impl GameEventBus for NatsEventBus {
    async fn publish_event(&self, topic: &str, payload: Vec<u8>) -> anyhow::Result<()> {
        self.client
            .read()
            .await
            .publish(topic.to_string(), payload.into())
            .await?;
        Ok(())
    }

    async fn subscribe<F>(&self, topic: &str, handler: F) -> anyhow::Result<()>
    where
        F: Fn(Vec<u8>) + Send + Sync + 'static,
    {
        let sub = self.client.read().await.subscribe(topic.to_string()).await?;
        tokio::spawn(async move {
            let mut sub = sub;
            while let Some(msg) = sub.next().await {
                handler(msg.payload.to_vec());
            }
        });
        Ok(())
    }
}