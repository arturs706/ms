use crate::properties::{
    domain_layer::property_address::PropertyAddress,
    infrastructure_layer::property_address_repository::PropertyAddressRepository,
};
use crate::AppState;
use actix_web::{web, HttpResponse, Responder};
use uuid::Uuid;

pub async fn get_all(state: web::Data<AppState>) -> impl Responder {
    let repo = PropertyAddressRepository::new();
    match repo.get_all(state.into_inner()).await {
        Ok(property_address) => HttpResponse::Ok().json(property_address),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

pub async fn add(
    state: web::Data<AppState>,
    property_address: web::Json<PropertyAddress>,
) -> impl Responder {
    let repo = PropertyAddressRepository::new();
    match repo
        .save(state.into_inner(), property_address.into_inner())
        .await
    {
        Ok(property_address) => HttpResponse::Ok().json(property_address),
        Err(e) => HttpResponse::InternalServerError().json(e),
    }
}

pub async fn get_by_id(state: web::Data<AppState>, property_id: web::Path<Uuid>) -> impl Responder {
    let repo = PropertyAddressRepository::new();
    match repo
        .get_by_id(state.into_inner(), property_id.into_inner())
        .await
    {
        Ok(property_address) => HttpResponse::Ok().json(property_address),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

pub async fn update(
    state: web::Data<AppState>,
    property_address: web::Json<PropertyAddress>,
) -> impl Responder {
    let repo = PropertyAddressRepository::new();
    match repo
        .update(state.into_inner(), property_address.into_inner())
        .await
    {
        Ok(property_address) => HttpResponse::Ok().json(property_address),
        Err(e) => HttpResponse::InternalServerError().json(e),
    }
}
