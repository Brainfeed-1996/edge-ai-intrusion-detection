use crate::domain::{ThreatIntel, Severity};
use crate::adapters::scraper;
use chrono::Utc;
use uuid::Uuid;

pub struct AggregatorService;

impl AggregatorService {
    pub async fn process_new_intel(url: &str) -> Result<ThreatIntel, String> {
        let raw_content = scraper::fetch_intel(url).await?;
        
        // Pseudo NLP logic
        let severity = if raw_content.contains("critical") {
            Severity::Critical
        } else {
            Severity::Medium
        };

        let intel = ThreatIntel {
            id: Uuid::new_v4(),
            source: url.to_string(),
            content: raw_content,
            severity,
            timestamp: Utc::now(),
            tags: vec!["aggregated".to_string()],
        };

        Ok(intel)
    }
}
