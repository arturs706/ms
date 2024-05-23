

use std::fmt;
use actix_web::{HttpResponse, error::ResponseError};
use serde_json::json;


#[derive(Debug)]
pub enum CustomErrors {
    #[allow(dead_code)]
    NoUsersFound,
    #[allow(dead_code)]
    MissingCreds,
    #[allow(dead_code)]
    InvalidToken,
    #[allow(dead_code)]
    NotLoggedIn,
    #[allow(dead_code)]
    InvalidKey,
    #[allow(dead_code)]
    NotAuthorized,
    #[allow(dead_code)]
    DatabaseError,
    #[allow(dead_code)]
    InternalServerError,
}

impl fmt::Display for CustomErrors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::NoUsersFound => write!(f, "No users found"),
            Self::MissingCreds => write!(f, "Missing credentials"),
            Self::NotLoggedIn => write!(f, "Error"),
            Self::InvalidToken => write!(f, "Invalid token"),
            Self::InvalidKey => write!(f, "Invalid key"),
            Self::NotAuthorized => write!(f, "Not authorized"),
            Self::DatabaseError => write!(f, "Database error"),
            Self::InternalServerError => write!(f, "Internal server error"),
        }
    }
}

impl ResponseError for CustomErrors {
    fn error_response(&self) -> HttpResponse {
        let status = match self {
            Self::NoUsersFound => actix_web::http::StatusCode::NOT_FOUND,
            Self::MissingCreds => actix_web::http::StatusCode::BAD_REQUEST,
            Self::NotLoggedIn => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            Self::InvalidToken => actix_web::http::StatusCode::UNAUTHORIZED,
            Self::InvalidKey => actix_web::http::StatusCode::UNAUTHORIZED,
            Self::NotAuthorized => actix_web::http::StatusCode::UNAUTHORIZED,
            Self::DatabaseError => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            Self::InternalServerError => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
        };

        HttpResponse::build(status)
            .json(json!({ "error": self.to_string() }))
    }
}
