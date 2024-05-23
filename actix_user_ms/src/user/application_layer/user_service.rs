use crate::user::{
    domain_layer::user::StaffUser,
    infrastructure_layer::{jwt_repo, user_repository::UserRepository}
};
use actix_web::{http::header::{self, HeaderMap}, web, HttpResponse, Responder};
use http::header::HeaderName;
use serde_json::json;
use uuid::Uuid;
use crate::AppState;

pub async fn get_all_users(state: web::Data<AppState>) -> impl Responder {
    let repo = UserRepository::new();
    match repo.get_all(state.into_inner()).await {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

pub async fn get_user_by_id(state: web::Data<AppState>, user_id: web::Path<Uuid>) -> impl Responder {
    let repo = UserRepository::new();
    match repo.get_by_id(state.into_inner(), user_id.into_inner()).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}


pub async fn register_user(state: web::Data<AppState>, user: web::Json<StaffUser>) -> impl Responder {
    let repo = UserRepository::new();
    match repo.save(state.into_inner(), user.into_inner()).await {
        Ok(saved_user) => HttpResponse::Ok().json(saved_user),
        Err(e) => HttpResponse::InternalServerError().json(e),
    }
}

pub async fn update_user(state: web::Data<AppState>, user: web::Json<StaffUser>) -> impl Responder {
    let repo = UserRepository::new();
    match repo.update(state.into_inner(), user.into_inner()).await {
        Ok(updated_user) => HttpResponse::Ok().json(updated_user),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

pub async fn delete_user(state: web::Data<AppState>, user_id: web::Path<Uuid>) -> impl Responder {
    let repo = UserRepository::new();
    match repo.delete(state.into_inner(), user_id.into_inner()).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

pub async fn login_user(state: web::Data<AppState>, user: web::Json<StaffUser>) -> impl Responder {
    let repo = UserRepository::new();
    let access_token_result = jwt_repo::create_token(&user, "user", "access", &state).await;
    let refresh_token_result = jwt_repo::create_token(&user, "user", "refresh", &state).await;

    if user.passwd.is_empty() {
        return HttpResponse::BadRequest().json(json!({"error": "Password is required"}));
    }

    match repo.login(state.into_inner(), user.into_inner()).await {
        Ok(user) => {
            let mut response = HttpResponse::Ok().json(user);
            
            if let Ok(access_token) = access_token_result {
                response.headers_mut().insert(
                    header::HeaderName::from_static("access_token"),
                    header::HeaderValue::from_str(&access_token).unwrap(),
                );
            } else {
                return HttpResponse::InternalServerError().json("Error creating access token");
            }
            
            if let Ok(refresh_token) = refresh_token_result {
                response.headers_mut().insert(
                    header::HeaderName::from_static("refresh_token"),
                    header::HeaderValue::from_str(&refresh_token).unwrap(),
                );
            } else {
                return HttpResponse::InternalServerError().json("Error creating refresh token");
            }
            
            response
        },
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}
