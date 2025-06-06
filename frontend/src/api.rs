use common::{LeaderboardEntry, SubmitScoreRequest, TopUserSchoolEntry, config::API_BASE_URL};
use gloo_net::http::Request;
use serde::de::DeserializeOwned;
use thiserror::Error;
use uuid::Uuid;
use web_sys::js_sys::encode_uri_component;

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
        _ if (200..300).contains(&status) => Ok(()), // Other success codes just in case
        _ => {
            let message = response
                .text()
                .await
                .unwrap_or_else(|_| "Failed to get error message".to_string());
            Err(ApiError::Server { status, message })
        }
    }
}

// Ensure this is the version you have:
pub async fn fetch_leaderboard(
    course: &str,
    school: &str,
    school_id: &Uuid,
) -> Result<Vec<LeaderboardEntry>, ApiError> {
    let encoded_course = encode_uri_component(course.trim().to_lowercase().as_str());
    let encoded_school = encode_uri_component(school); // Encode school name
    let school_id_str = school_id.to_string(); // Convert Uuid to string

    // *** THIS is the critical line ***
    let url = if API_BASE_URL.is_empty() {
        // In production, use relative URLs that work regardless of domain
        format!(
            "/api/leaderboard?course={}&school={}&school_id={}",
            encoded_course, encoded_school, school_id_str
        )
    } else {
        // In development, use the full URL with localhost
        format!(
            "{}/api/leaderboard?course={}&school={}&school_id={}",
            API_BASE_URL, encoded_course, encoded_school, school_id_str
        )
    };
    // Optional: Keep this log for debugging
    web_sys::console::log_1(&format!("Fetching leaderboard from URL: {}", url).into());

    let response = Request::get(&url).send().await?;

    handle_response(response).await
}

pub async fn submit_score(req: &SubmitScoreRequest) -> Result<(), ApiError> {
    let url = if API_BASE_URL.is_empty() {
        "/api/submit".to_string()
    } else {
        format!("{}/api/submit", API_BASE_URL)
    };
    web_sys::console::log_1(&format!("Submitting score to URL: {}", url).into()); // Debug log
    web_sys::console::log_1(&format!("Submit request payload: {:?}", req).into()); // Debug log

    let response = Request::post(&url)
        .json(req)
        .map_err(|e| ApiError::Build(format!("Failed to serialize request: {}", e)))?
        .send()
        .await?;

    handle_submit_response(response).await
}
pub async fn fetch_top_users_by_school(
    school_name: &str,
    school_id_uuid: &Uuid,
    limit_count: u32, // This is the limit passed to the API
) -> Result<Vec<TopUserSchoolEntry>, ApiError> {
    // Expect TopUserSchoolEntry
    let encoded_school_name = encode_uri_component(school_name);
    let school_id_str = school_id_uuid.to_string();

    let url = if API_BASE_URL.is_empty() {
        format!(
            "/api/top_users_by_school?school={}&school_id={}&limit={}", // Pass limit
            encoded_school_name, school_id_str, limit_count
        )
    } else {
        format!(
            "{}/api/top_users_by_school?school={}&school_id={}&limit={}", // Pass limit
            API_BASE_URL, encoded_school_name, school_id_str, limit_count
        )
    };

    web_sys::console::log_1(&format!("Fetching top users by school from URL: {}", url).into());

    let response = Request::get(&url).send().await?;
    handle_response(response).await
}
