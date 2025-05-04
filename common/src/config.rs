#[cfg(debug_assertions)]
pub const API_BASE_URL: &str = "http://127.0.0.1:8080";
#[cfg(not(debug_assertions))]
pub const API_BASE_URL: &str = "";
pub const MAX_ENTRIES_PER_COURSE: i64 = 10;
