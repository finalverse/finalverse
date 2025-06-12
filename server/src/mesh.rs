use std::{collections::HashMap, net::SocketAddr, sync::Arc, time::Duration};
use std::net::AddrParseError;
use tokio::sync::RwLock;
use tonic::transport::{Channel, Endpoint};
use uuid::Uuid;
use anyhow::Result;
use once_cell::sync::Lazy;

#[derive(Debug, Clone)]
pub struct MeshContext {
    pub request_id: Uuid,
    pub peer_cert: Option<String>,
    pub trace_id: Uuid,
}

#[derive(Clone, Default)]
pub struct GrpcAddressBook {
    inner: Arc<RwLock<HashMap<String, SocketAddr>>>,
}

impl GrpcAddressBook {
    pub fn new() -> Self {
        Self { inner: Arc::new(RwLock::new(HashMap::new())) }
    }
    pub async fn update(&self, map: HashMap<String, SocketAddr>) {
        let mut guard = self.inner.write().await;
        *guard = map;
    }
    pub async fn get(&self, name: &str) -> Option<SocketAddr> {
        self.inner.read().await.get(name).cloned()
    }
}

pub static ADDRESS_BOOK: Lazy<GrpcAddressBook> = Lazy::new(GrpcAddressBook::new);

pub async fn dial(service_name: &str) -> Result<Channel> {
    let addr = ADDRESS_BOOK
        .get(service_name)
        .await
        .ok_or_else(|| anyhow::anyhow!("unknown service: {service_name}"))?;
    let endpoint = Endpoint::from_shared(format!("http://{}", addr))?;
    Ok(endpoint.connect().await?)
}

pub fn spawn_refresh_task() {
    let book = ADDRESS_BOOK.clone();
    tokio::spawn(async move {
        loop {
            if let Ok(map) = fetch_address_book().await {
                book.update(map).await;
            }
            tokio::time::sleep(Duration::from_secs(30)).await;
        }
    });
}

async fn fetch_address_book() -> Result<HashMap<String, SocketAddr>> {
    let base = std::env::var("FINALVERSE_CONFIG_URL")
        .unwrap_or_else(|_| "http://localhost:7070".to_string());
    let resp = reqwest::get(format!("{}/services/grpc", base)).await?;
    let raw: HashMap<String, String> = resp.json().await?;
    raw.into_iter()
        .map(|(k, v)| v.parse().map(|a| (k, a)))
        .collect::<std::result::Result<_, _>>()
        .map_err(|e: AddrParseError | anyhow::anyhow!(e))
}
