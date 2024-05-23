use http::HeaderValue as HTTPOneHeaderValue;
use jsonwebtoken::{Header as JsonWebTokenHeader, Algorithm, EncodingKey};
use serde::{Deserialize, Serialize};
use chrono::{Utc, Duration};
use axum::{
    http::Request,
    middleware::Next,
    response::Response,
};
use tonic::{Request as TonicRequest, Status};

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthClaims {
    iss: String,
    exp: i64,
    iat: i64,
    role: String,
}

impl AuthClaims {
    pub fn new(issuer: String) -> Self {
        let iat = Utc::now();
        let exp = iat + Duration::hours(72);

        Self {
            iss: issuer,
            exp: exp.timestamp(),
            iat: iat.timestamp(),
            role: "Other".to_string(),
        }
    }
}

pub async fn add_token<B>(req: Request<B>, next: Next<B>) -> Result<Response, Response> {
    let issuer = "APIGW".to_string();
    let jwt_secret: String = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let jwt_token = jsonwebtoken::encode(
        &JsonWebTokenHeader::new(Algorithm::HS256),
        &AuthClaims::new(issuer.clone()),
        &EncodingKey::from_secret(jwt_secret.as_bytes()),
    ).unwrap();
    let mut req = req;
    req.headers_mut().insert("Authorization", HTTPOneHeaderValue::from_str(&jwt_token).unwrap());
    let response = next.run(req).await;
    Ok(response)
}




pub fn add_token_to_request<T>(
    mut req: TonicRequest<T>,
) -> TonicRequest<T> {
    let issuer = "APIGW".to_string();
    let jwt_secret: String = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let jwt_token = jsonwebtoken::encode(
        &JsonWebTokenHeader::new(Algorithm::HS256),
        &AuthClaims::new(issuer.clone()),
        &EncodingKey::from_secret(jwt_secret.as_bytes()),
    ).unwrap();
    let metadata = req.metadata_mut();

    let token = format!("Bearer {}", jwt_token);
    metadata.insert("authorization", token.parse().unwrap());
    req
}