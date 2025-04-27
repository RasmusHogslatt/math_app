pub mod config;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LeaderboardEntry {
    pub id: i32,
    pub name: String,
    pub course: String,
    pub school: String,
    pub school_id: uuid::Uuid,
    pub time_seconds: f64,
    pub completed_at: Option<DateTime<Utc>>, // This is still optional as default value is handled in the database
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaderboardRequest {
    pub course: String,
    pub school: String,
    pub school_id: uuid::Uuid,
    pub limit: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitScoreRequest {
    pub name: String,
    pub course: String,
    pub school: String,
    pub school_id: uuid::Uuid,
    pub time_seconds: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct User {
    pub name: String,
    pub school: String,
    pub school_id: uuid::Uuid,
}

impl User {
    pub fn new_dummy() -> Self {
        Self {
            name: "Dummy".to_string(),
            school: "Donald Duck University".to_string(),
            school_id: uuid::Uuid::new_v4(),
        }
    }
}
