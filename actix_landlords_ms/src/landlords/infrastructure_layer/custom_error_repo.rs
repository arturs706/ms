use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use derive_more::Display;
use serde::Serialize;
use serde_json::json;


#[allow(dead_code)]
#[derive(Debug, Display, Serialize)]
pub enum CustomErrors {
    #[display(fmt = "No users found")]
    NoUsersFound,
    #[display(fmt = "Missing credentials")]
    MissingCreds,
    #[display(fmt = "Invalid token")]
    InvalidToken,
    #[display(fmt = "Not logged in")]
    NotLoggedIn,
    #[display(fmt = "Invalid key")]
    InvalidKey,
    #[display(fmt = "Not authorized")]
    NotAuthorized,
    #[display(fmt = "Database error")]
    DatabaseError,
    #[display(fmt = "Internal server error")]
    InternalServerError,
    #[display(fmt = "Duplicate key error")]
    DuplicateKeyError

}

impl ResponseError for CustomErrors {
    fn error_response(&self) -> HttpResponse {
        let status_code = match self {
            CustomErrors::NoUsersFound => StatusCode::NOT_FOUND,
            CustomErrors::MissingCreds => StatusCode::BAD_REQUEST,
            CustomErrors::NotLoggedIn => StatusCode::INTERNAL_SERVER_ERROR,
            CustomErrors::InvalidToken => StatusCode::UNAUTHORIZED,
            CustomErrors::InvalidKey => StatusCode::UNAUTHORIZED,
            CustomErrors::NotAuthorized => StatusCode::UNAUTHORIZED,
            CustomErrors::DatabaseError => StatusCode::INTERNAL_SERVER_ERROR,
            CustomErrors::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            CustomErrors::DuplicateKeyError => StatusCode::CONFLICT,
        };

        let error_message = self.to_string();

        HttpResponse::build(status_code)
            .json(json!({ "error": error_message }))
    }
}