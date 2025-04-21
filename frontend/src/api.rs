use common::{LeaderboardEntry, SubmitScoreRequest, config::API_BASE_URL};
use gloo_net::http::Request;
use serde::de::DeserializeOwned;
use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum ApiError {
    #[error("Network error: {0}")]
    Network(String),
    #[error("Server error: status {status}, message: {message}")]
    Server { status: u16, message: String },
    #[error("Failed to parse response: {0}")]
    Parse(String),
    #[error("Failed to build request: {0}")]
    Build(String),
    #[error("Conflict: {0}")] // Specific for 409
    Conflict(String),
}

// Helper to convert gloo_net::Error
impl From<gloo_net::Error> for ApiError {
    fn from(err: gloo_net::Error) -> Self {
        ApiError::Network(err.to_string())
    }
}

async fn handle_response<T: DeserializeOwned>(
    response: gloo_net::http::Response,
) -> Result<T, ApiError> {
    let status = response.status();
    if response.ok() {
        // 2xx status codes
        response
            .json::<T>()
            .await
            .map_err(|e| ApiError::Parse(e.to_string()))
    } else {
        let message = response
            .text()
            .await
            .unwrap_or_else(|_| "Failed to get error message".to_string());
        Err(ApiError::Server { status, message })
    }
}

async fn handle_submit_response(response: gloo_net::http::Response) -> Result<(), ApiError> {
    let status = response.status();
    match status {
        201 => Ok(()), // Created
        409 => {
            let message = response
                .text()
                .await
                .unwrap_or_else(|_| "Score already submitted.".to_string());
            Err(ApiError::Conflict(message))
        }
        _ if status >= 200 && status < 300 => Ok(()), // Other success codes just in case
        _ => {
            let message = response
                .text()
                .await
                .unwrap_or_else(|_| "Failed to get error message".to_string());
            Err(ApiError::Server { status, message })
        }
    }
}

pub async fn fetch_leaderboard(course: &str) -> Result<Vec<LeaderboardEntry>, ApiError> {
    let url = format!(
        "{}/api/leaderboard?course={}",
        API_BASE_URL,
        course.trim().to_lowercase()
    );
    let response = Request::get(&url).send().await?; // Propagates ApiError::Network

    handle_response(response).await
}

pub async fn submit_score(req: &SubmitScoreRequest) -> Result<(), ApiError> {
    let url = format!("{}/api/submit", API_BASE_URL);
    let response = Request::post(&url)
        .json(req)
        .map_err(|e| ApiError::Build(format!("Failed to serialize request: {}", e)))?
        .send()
        .await?; // Propagates ApiError::Network

    handle_submit_response(response).await
}
