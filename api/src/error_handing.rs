use std::fmt::Display;

use axum::{http::StatusCode, response::IntoResponse};

#[derive(Debug)]
pub enum AppErr {
    DatabaseErr(String),
    NotFound(String),
    Other(String),
}

impl Display for AppErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppErr::DatabaseErr(msg) => write!(f, "Database : {}", msg),
            AppErr::NotFound(msg) => write!(f, "NotFound : {}", msg),
            AppErr::Other(msg) => write!(f, "Other : {}", msg),
        }
    }
}

impl std::error::Error for AppErr {}

impl IntoResponse for AppErr {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            AppErr::DatabaseErr(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            AppErr::NotFound(_) => (StatusCode::NOT_FOUND, self.to_string()),
            AppErr::Other(_) => (StatusCode::BAD_REQUEST, self.to_string()),
        };
        (status, error_message).into_response()
    }
}
