use async_trait::async_trait;

#[async_trait]
pub trait GameEventBus {
    async fn publish_event(&self, topic: &str, payload: Vec<u8>) -> anyhow::Result<()>;
    async fn subscribe<F>(&self, topic: &str, handler: F) -> anyhow::Result<()>
    where
        F: Fn(Vec<u8>) + Send + Sync + 'static;
}
