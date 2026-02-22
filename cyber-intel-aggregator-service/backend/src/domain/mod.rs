use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ThreatIntel {
    pub id: uuid::Uuid,
    pub source: String,
    pub content: String,
    pub severity: Severity,
    pub timestamp: DateTime<Utc>,
    pub tags: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Severity {
    Low,
    Medium,
    High,
    Critical,
}

pub trait IntelRepository: Send + Sync {
    fn save(&self, intel: ThreatIntel) -> Result<(), String>;
    fn list(&self) -> Result<Vec<ThreatIntel>, String>;
}
