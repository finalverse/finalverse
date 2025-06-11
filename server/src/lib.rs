use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServiceStatus {
    Starting,
    Running,
    Stopping,
    Stopped,
    Error(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub timestamp: DateTime<Utc>,
    pub level: LogLevel,
    pub service: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceInfo {
    pub name: String,
    pub port: u16,
    pub status: ServiceStatus,
    pub pid: Option<u32>,
    pub uptime: Duration,
    pub last_health_check: Option<DateTime<Utc>>,
    pub health_status: bool,
    pub cpu_usage: f32,
    pub memory_usage: u64,
    pub log_lines: VecDeque<LogEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServerCommand {
    StartService(String),
    StopService(String),
    RestartService(String),
    GetServiceStatus(String),
    GetAllServices,
    GetLogs { service: Option<String>, lines: usize },
    ExecuteCommand(String),
    Shutdown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServerResponse {
    ServiceStatus(ServiceInfo),
    AllServices(Vec<ServiceInfo>),
    Logs(Vec<LogEntry>),
    CommandResult(String),
    Error(String),
    Ok,
}
