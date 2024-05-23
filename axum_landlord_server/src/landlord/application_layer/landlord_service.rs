use crate::landlord::{
    domain_layer::landlord::Landlord,
    infrastructure_layer::{
        custom_error_repo::CustomErrors, landlord_repository::LandlordRepository,
    },
};
use axum::{
    extract::Path,
    http::{HeaderMap, StatusCode},
    Json,
};
use axum_macros::debug_handler;
use uuid::Uuid;

#[debug_handler]
pub async fn get_all_landlords() -> Result<Json<Vec<Landlord>>, (StatusCode, String)> {
    let landlord_repository = LandlordRepository::new().await;
    match landlord_repository.get_all().await {
        Ok(landlords) => Ok(Json(landlords)),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
    }
}

#[debug_handler]
pub async fn get_landlord_by_id(
    Path(landlord_id): Path<Uuid>,
) -> Result<Json<Landlord>, (StatusCode, String)> {
    let landlord_repository = LandlordRepository::new().await;
    match landlord_repository.get_by_id(landlord_id).await {
        Ok(landlord) => Ok(Json(landlord)),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
    }
}

#[debug_handler]
pub async fn save_landlord(
    headers: HeaderMap,
    landlord: Json<Landlord>
) -> Result<Json<Landlord>, (StatusCode, String)> {
    let token = headers
        .get("authorization")
        .ok_or(CustomErrors::NotAuthorized);
    // let token = token.to_str().map_err(|_| CustomErrors::Unauthorized)?;
    // let user = jwt_repository::decode_token(token).await?;
    let landlord_repository = LandlordRepository::new().await;
    let mut landlord = landlord.0;
    // landlord.createdby = user.username;
    match landlord_repository.save(landlord).await {
        Ok(landlord) => Ok(Json(landlord)),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
    }
}