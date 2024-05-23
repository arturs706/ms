#![allow(dead_code)]


use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

#[derive(Debug)]

pub enum CustomErrors {
    NoUsersFound,
    MissingCreds,
    InvalidToken,
    NotLoggedIn,
    InvalidKey,
    NotAuthorized,
    DatabaseError,
    InternalServerError,
}

impl IntoResponse for CustomErrors {
    fn into_response(self) -> axum::response::Response {
        let (status, err_msg) = match self {
            Self::NoUsersFound => (StatusCode::NOT_FOUND, "No users found"),
            Self::MissingCreds => (StatusCode::BAD_REQUEST, "Missing credentials"),
            Self::NotLoggedIn => (StatusCode::INTERNAL_SERVER_ERROR, "Error"),
            Self::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid token"),
            Self::InvalidKey => (StatusCode::UNAUTHORIZED, "Invalid key"),
            Self::NotAuthorized => (StatusCode::UNAUTHORIZED, "Not authorized"),
            Self::DatabaseError => (StatusCode::INTERNAL_SERVER_ERROR, "Database error"),
            Self::InternalServerError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
        };
        (status, Json(json!({ "error": err_msg}))).into_response()
    }
}
