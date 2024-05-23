use crate::landlords::{
    domain_layer::{landlord::Landlord, landlord_address::LandlordAddress},
    infrastructure_layer::landlords_repository::LandlordsRepository,
};
use actix_web::{ web, HttpResponse, Responder};
use uuid::Uuid;
use crate::AppState;

pub async fn get_all(state: web::Data<AppState>) -> impl Responder {
    let repo = LandlordsRepository::new();
    match repo.get_all(state.into_inner()).await {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

pub async fn add(state: web::Data<AppState>, landlord: web::Json<Landlord>) -> impl Responder {
    let repo = LandlordsRepository::new();
    match repo.save(state.into_inner(), landlord.into_inner()).await {
        Ok(landlord) => HttpResponse::Ok().json(landlord),
        Err(e) => HttpResponse::InternalServerError().json(e),
    }
}

pub async fn add_address(state: web::Data<AppState>, path: web::Path<Uuid>, address: web::Json<LandlordAddress>) -> impl Responder {
    println!("add_address");
    let landlord_id = path.into_inner();
    let repo = LandlordsRepository::new();
    match repo.saveaddress(state.into_inner(), landlord_id, address.into_inner()).await {
        Ok(landlord) => HttpResponse::Ok().json(landlord),
        Err(e) => HttpResponse::InternalServerError().json(e),
    }
    
}

pub async fn delete_landlord(state: web::Data<AppState>, path: web::Path<Uuid>) -> impl Responder {
    let landlord_id = path.into_inner();
    let repo = LandlordsRepository::new();
    match repo.delete_landlord(state.into_inner(), landlord_id).await {
        Ok(landlord) => HttpResponse::Ok().json(landlord),
        Err(e) => HttpResponse::InternalServerError().json(e),
    }
}