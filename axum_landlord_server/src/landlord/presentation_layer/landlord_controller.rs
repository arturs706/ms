use axum::{
    routing::{get, post},
    Router,
};

use crate::landlord::application_layer::landlord_service;

pub fn create_router() -> Router {
    Router::new()
        .route("/api/v1/landlords", get(landlord_service::get_all_landlords))
        .route("/api/v1/landlords/add", post(landlord_service::save_landlord))
        .route("/api/v1/landlords/:id", get(landlord_service::get_landlord_by_id))
}
