use super::custom_error_repo::CustomErrors;
use super::jwt_repo::validate_token;
use axum::body::Body;
use axum::{http::Request, middleware::Next, response::Response};
use jsonwebtoken::errors::ErrorKind;

pub async fn auth_middleware(request: Request<Body>, next: Next) -> Result<Response, CustomErrors>
where
    Body: Send,
{
    let auth = request
        .headers()
        .get("Authorization")
        .ok_or(CustomErrors::MissingCreds)?;
    let token = auth.to_str().map_err(|_| CustomErrors::MissingCreds)?;
    let authtoken = token.replace("Bearer ", "");
    match validate_token(&authtoken).await {
        Ok(_) => Ok(next.run(request).await),
        Err(e) => {
            println!("Access verify error: {:?}", e);
            match e.kind() {
                ErrorKind::InvalidToken => Err(CustomErrors::InvalidToken),
                _ => Err(CustomErrors::InvalidKey),
            }
        }
    }
}
