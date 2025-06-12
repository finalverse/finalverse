// crates/fv-events/src/event_bus.rs
use async_trait::async_trait;
use crate::events::Event;

#[async_trait]
pub trait GameEventBus: Send + Sync {
    /// Publish raw bytes to a topic
    async fn publish_raw(&self, topic: &str, payload: Vec<u8>) -> anyhow::Result<()>;
    
    /// Subscribe to raw bytes from a topic
    async fn subscribe_raw<F>(&self, topic: &str, handler: F) -> anyhow::Result<String>
    where
        F: Fn(Vec<u8>) + Send + Sync + 'static;
    
    /// Publish a typed event
    async fn publish(&self, event: Event) -> anyhow::Result<()> {
        let topic = event.topic();
        let payload = serde_json::to_vec(&event)?;
        self.publish_raw(&topic, payload).await
    }
    
    /// Subscribe to typed events
    async fn subscribe<F>(&self, topic: &str, handler: F) -> anyhow::Result<String>
    where
        F: Fn(Event) + Send + Sync + 'static,
    {
        let topic = topic.to_string();
        self.subscribe_raw(&topic, move |payload| {
            if let Ok(event) = serde_json::from_slice::<Event>(&payload) {
                handler(event);
            }
        }).await
    }
    
    /// Unsubscribe from a topic
    async fn unsubscribe(&self, subscription_id: &str) -> anyhow::Result<()>;
}