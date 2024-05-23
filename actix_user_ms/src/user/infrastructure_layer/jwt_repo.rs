use actix_web::web;
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::env;

use crate::{
    user::domain_layer::user::StaffUser,
    AppState,
};


#[derive(Debug, Serialize, Deserialize)]
pub struct AuthClaims {
    iss: AuthType,
    exp: i64,
    iat: i64,
    role: Issuer,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AuthType {
    APIGW,
    Access,
    Refresh,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Issuer {
    Admin,
    User,
    Other,
}

impl AuthType {
    pub fn new(auth_type: &str) -> Self {
        match auth_type {
            "access" => Self::Access,
            "refresh" => Self::Refresh,
            _ => Self::APIGW,
        }
    }
}

impl Issuer {
    pub fn _new(issuer: &str) -> Self {
        match issuer {
            "admin" => Self::Admin,
            "user" => Self::User,
            _ => Self::Other,
        }
    }
}

impl AuthClaims {
    pub fn new(_issuer: &str, role: Issuer, auth_type: &str) -> Self {
        let iat = Utc::now().timestamp();
        let exp = match auth_type {
            "refresh" => (Utc::now() + Duration::days(30)).timestamp(),
            _ => (Utc::now() + Duration::minutes(15)).timestamp(),
        };

        AuthClaims {
            iss: AuthType::new(auth_type),
            exp,
            iat,
            role,
        }
    }
}

#[allow(dead_code)]
pub async fn create_token(
    user: &StaffUser,
    issuer: &str,
    auth_type: &str,
    app_state: &web::Data<AppState>,
) -> Result<String, jsonwebtoken::errors::Error> {
    let app_state = app_state.get_ref().clone();
    let jwt_secret: String = env::var("JWT_SECRET").unwrap_or_else(|_| app_state.jwt_secret.clone());
    let access_secret = jwt_secret.as_bytes();
    let role = if user.username == "arturs" {
        Issuer::Admin
    } else {
        Issuer::User
    };
    let claims = AuthClaims::new(issuer, role, auth_type);
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(access_secret),
    )?;
    Ok(token)
}