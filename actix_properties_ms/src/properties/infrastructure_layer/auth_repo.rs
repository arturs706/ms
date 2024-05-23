use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::{error::ErrorUnauthorized, Error, Result};
use chrono::{Duration, Utc};
use futures_util::future::LocalBoxFuture;
use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::future::{ready, Ready};

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

pub struct Auth;
impl<S, B> Transform<S, ServiceRequest> for Auth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddleware { service }))
    }
}

pub struct AuthMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, request: ServiceRequest) -> Self::Future {
        dotenv::dotenv().ok();
        let access_token_secret: String =
            std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        if let Some(auth) = request.headers().get("Authorization") {
            if let Ok(token) = auth.to_str() {
                let authtoken = token.replace("Bearer ", "");
                let validation = Validation::new(Algorithm::HS256);
                let access_secret = access_token_secret.as_bytes();
                match jsonwebtoken::decode::<AuthClaims>(
                    &authtoken,
                    &DecodingKey::from_secret(access_secret),
                    &validation,
                ) {
                    Ok(_) => {
                        return Box::pin(self.service.call(request));
                    }
                    Err(e) => {
                        println!("access_verify: {:?}", e);
                        match e.kind() {
                            ErrorKind::ExpiredSignature => {
                                return Box::pin(ready(Err(ErrorUnauthorized("Token Expired"))));
                            }
                            _ => {
                                return Box::pin(ready(Err(ErrorUnauthorized("Invalid Token"))));
                            }
                        }
                    }
                }
            } else {
                return Box::pin(ready(Err(ErrorUnauthorized("Missing Creds"))));
            }
        } else {
            return Box::pin(ready(Err(ErrorUnauthorized("Not Logged In"))));
        }
    }
}
