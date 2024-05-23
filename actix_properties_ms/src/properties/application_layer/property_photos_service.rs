use std::fmt;

use crate::properties::{
    domain_layer::property_photos::PropertyPhotos,
    infrastructure_layer::property_photos_repository::PropertyPhotosRepository,
};
use crate::AppState;
use actix_multipart::form::bytes;
use actix_multipart::Multipart;
use actix_web::http::{Error, StatusCode};
use actix_web::web::Json;
use actix_web::{web, HttpRequest, HttpResponse, Responder, ResponseError};
use std::{error::Error as stdError, str};

use serde_json::json;
use uuid::Uuid;


pub async fn get_all(state: web::Data<AppState>) -> impl Responder {
    let repo = PropertyPhotosRepository::new();

    match repo.get_all(state.into_inner()).await {
        Ok(property_photos) => HttpResponse::Ok().json(property_photos),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

pub async fn add(
    state: web::Data<AppState>,
    property_photos: web::Json<PropertyPhotos>,
) -> impl Responder {
    let repo = PropertyPhotosRepository::new();

    match repo
        .save(state.into_inner(), property_photos.into_inner())
        .await
    {
        Ok(property_photos) => HttpResponse::Ok().json(property_photos),
        Err(e) => HttpResponse::InternalServerError().json(e),
    }
}

pub async fn get_by_id(state: web::Data<AppState>, property_id: web::Path<Uuid>) -> impl Responder {
    let repo = PropertyPhotosRepository::new();

    match repo
        .get_by_id(state.into_inner(), property_id.into_inner())
        .await
    {
        Ok(property_photos) => HttpResponse::Ok().json(property_photos),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}
pub async fn upload_images(
    state: web::Data<AppState>,
    property_id: web::Path<Uuid>,
    mut payload: Multipart,
    req: HttpRequest,
) -> impl Responder {
    let repo = PropertyPhotosRepository::new();
    println!("upload_images");
    match repo
        .upload_images(
            state.into_inner(),
            property_id.into_inner(),
            &mut payload,
            req.into(),
        )
        .await
    {
        Ok((status_code, property_photos)) => {
            HttpResponse::build(status_code).json(property_photos)
        }
        Err((status_code, e)) => HttpResponse::build(status_code).json(e),
    }
}

pub async fn testHello() -> impl Responder {
    HttpResponse::Ok().body("Hello")
}
   
