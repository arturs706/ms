use actix_web::web;

use crate::user::application_layer::user_service;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .route("/users", web::get().to(user_service::get_all_users))
            .route("/users/{user_id}", web::get().to(user_service::get_user_by_id))
            .route("/users", web::post().to(user_service::register_user))
            .route("/users", web::put().to(user_service::update_user))
            .route("/users/{user_id}", web::delete().to(user_service::delete_user))
            .route("/users/login", web::post().to(user_service::login_user))
    );
}