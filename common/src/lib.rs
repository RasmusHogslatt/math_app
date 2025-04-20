pub mod config;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LeaderboardEntry {
    pub id: i32,
    pub name: String,
    pub course: String,
    pub time_seconds: f64,
    pub completed_at: Option<DateTime<Utc>>, // This is still optional as default value is handled in the database
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaderboardRequest {
    pub course: String,
    pub limit: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitScoreRequest {
    pub name: String,
    pub course: String,
    pub time_seconds: f64,
}
