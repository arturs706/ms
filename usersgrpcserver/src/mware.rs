use chrono::{Utc, Duration};
use tonic::{Request as TonicRequest, Status};
use serde::{Deserialize, Serialize};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use dotenv::dotenv;


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
    #[allow(dead_code)]
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
    #[allow(dead_code)]
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


pub fn check_auth(req: TonicRequest<()>) -> Result<TonicRequest<()>, Status> {
    dotenv().ok();

    match req.metadata().get("authorization") {
        Some(t) => {
            let split_token = t.to_str().unwrap();
            let split_token = split_token.split(" ").collect::<Vec<&str>>();
            let t = split_token[1].to_string();
            let validation = Validation::new(Algorithm::HS256);
            let jwt_secret: String = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
            let access_secret = jwt_secret.as_bytes();
            let access_verify = decode::<AuthClaims>(
                &t,
                &DecodingKey::from_secret(access_secret),
                &validation,
            )

            .map(|data| data.claims);
            match access_verify {
                Ok(claims) => {
                    if claims.exp < Utc::now().timestamp() {
                        return Err(Status::unauthenticated(
                            "Token has expired",
                        ));
                    }
                    Ok(req)
                }
                Err(e) => {
                    println!("{}", e);
                    Err(Status::unauthenticated(
                        "No valid auth token",
                    ))
                }
            }

        },
   
        _ => {
            Err(Status::unauthenticated(
                "No valid auth token",
            ))
        }
    }
}
