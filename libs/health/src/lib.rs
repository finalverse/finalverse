// libs/health/src/lib.rs
// Comprehensive health monitoring for Finalverse services

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use warp::{Filter, Rejection, Reply};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthStatus {
    pub service: String,
    pub version: String,
    pub status: ServiceStatus,
    pub uptime_seconds: u64,
    pub checks: Vec<HealthCheck>,
    pub metrics: HealthMetrics,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ServiceStatus {
    Healthy,
    Degraded,
    Unhealthy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheck {
    pub name: String,
    pub status: CheckStatus,
    pub message: Option<String>,
    pub latency_ms: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum CheckStatus {
    Pass,
    Warn,
    Fail,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthMetrics {
    pub requests_per_second: f64,
    pub average_response_time_ms: f64,
    pub error_rate: f64,
    pub active_connections: u64,
    pub memory_usage_mb: f64,
    pub cpu_usage_percent: f64,
}

impl Default for HealthMetrics {
    fn default() -> Self {
        Self {
            requests_per_second: 0.0,
            average_response_time_ms: 0.0,
            error_rate: 0.0,
            active_connections: 0,
            memory_usage_mb: 0.0,
            cpu_usage_percent: 0.0,
        }
    }
}

pub struct HealthMonitor {
    service_name: String,
    version: String,
    start_time: Instant,
    checks: Arc<RwLock<Vec<Box<dyn HealthChecker + Send + Sync>>>>,
    metrics: Arc<RwLock<HealthMetrics>>,
}

#[async_trait::async_trait]
pub trait HealthChecker: Send + Sync {
    async fn check(&self) -> HealthCheck;
    fn name(&self) -> &str;
}

// Basic connectivity checker
pub struct ConnectivityChecker {
    name: String,
    url: String,
}

impl ConnectivityChecker {
    pub fn new(name: String, url: String) -> Self {
        Self { name, url }
    }
}

#[async_trait::async_trait]
impl HealthChecker for ConnectivityChecker {
    async fn check(&self) -> HealthCheck {
        let start = Instant::now();
        let client = reqwest::Client::new();
        
        match client
            .get(&self.url)
            .timeout(Duration::from_secs(2))
            .send()
            .await
        {
            Ok(response) => {
                if response.status().is_success() {
                    HealthCheck {
                        name: self.name.clone(),
                        status: CheckStatus::Pass,
                        message: None,
                        latency_ms: Some(start.elapsed().as_millis() as u64),
                    }
                } else {
                    HealthCheck {
                        name: self.name.clone(),
                        status: CheckStatus::Warn,
                        message: Some(format!("HTTP {}", response.status())),
                        latency_ms: Some(start.elapsed().as_millis() as u64),
                    }
                }
            }
            Err(e) => HealthCheck {
                name: self.name.clone(),
                status: CheckStatus::Fail,
                message: Some(format!("Connection failed: {}", e)),
                latency_ms: None,
            },
        }
    }
    
    fn name(&self) -> &str {
        &self.name
    }
}

impl HealthMonitor {
    pub fn new(service_name: impl Into<String>, version: impl Into<String>) -> Self {
        Self {
            service_name: service_name.into(),
            version: version.into(),
            start_time: Instant::now(),
            checks: Arc::new(RwLock::new(Vec::new())),
            metrics: Arc::new(RwLock::new(HealthMetrics::default())),
        }
    }
    
    pub async fn add_checker(&self, checker: Box<dyn HealthChecker + Send + Sync>) {
        let mut checks = self.checks.write().await;
        checks.push(checker);
    }
    
    pub async fn update_metrics<F>(&self, updater: F)
    where
        F: FnOnce(&mut HealthMetrics),
    {
        let mut metrics = self.metrics.write().await;
        updater(&mut *metrics);
    }
    
    pub async fn get_status(&self) -> HealthStatus {
        let mut all_checks = Vec::new();
        let checks = self.checks.read().await;
        
        for checker in checks.iter() {
            all_checks.push(checker.check().await);
        }
        
        let status = if all_checks.iter().any(|c| c.status == CheckStatus::Fail) {
            ServiceStatus::Unhealthy
        } else if all_checks.iter().any(|c| c.status == CheckStatus::Warn) {
            ServiceStatus::Degraded
        } else {
            ServiceStatus::Healthy
        };
        
        let metrics = self.metrics.read().await.clone();
        
        HealthStatus {
            service: self.service_name.clone(),
            version: self.version.clone(),
            status,
            uptime_seconds: self.start_time.elapsed().as_secs(),
            checks: all_checks,
            metrics,
            timestamp: chrono::Utc::now(),
        }
    }
    
    pub fn create_routes(self: Arc<Self>) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
        let health = {
            let monitor = Arc::clone(&self);
            warp::path("health")
                .and(warp::get())
                .and_then(move || {
                    let monitor = Arc::clone(&monitor);
                    async move {
                        let status = monitor.get_status().await;
                        let status_code = match status.status {
                            ServiceStatus::Healthy => warp::http::StatusCode::OK,
                            ServiceStatus::Degraded => warp::http::StatusCode::OK,
                            ServiceStatus::Unhealthy => warp::http::StatusCode::SERVICE_UNAVAILABLE,
                        };
                        
                        Ok::<_, warp::Rejection>(
                            warp::reply::with_status(
                                warp::reply::json(&status),
                                status_code,
                            )
                        )
                    }
                })
        };
        
        let info = {
            let monitor = Arc::clone(&self);
            warp::path("info")
                .and(warp::get())
                .map(move || {
                    warp::reply::json(&ServiceInfo {
                        name: monitor.service_name.clone(),
                        version: monitor.version.clone(),
                        uptime_seconds: monitor.start_time.elapsed().as_secs(),
                    })
                })
        };
        
        health.or(info)
    }
}

#[derive(Debug, Serialize)]
pub struct ServiceInfo {
    pub name: String,
    pub version: String,
    pub uptime_seconds: u64,
}

// Convenience function to add standard checks
pub async fn add_standard_checks(monitor: &HealthMonitor, postgres_url: Option<&str>, redis_url: Option<&str>) {
    if let Some(pg_url) = postgres_url {
        monitor.add_checker(Box::new(ConnectivityChecker::new(
            "postgres".to_string(),
            format!("{}/health", pg_url.replace("postgres://", "http://").split('@').last().unwrap_or("localhost:5432")),
        ))).await;
    }
    
    if let Some(redis_url) = redis_url {
        monitor.add_checker(Box::new(ConnectivityChecker::new(
            "redis".to_string(),
            redis_url.replace("redis://", "http://"),
        ))).await;
    }
}