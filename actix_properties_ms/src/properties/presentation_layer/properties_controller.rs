use actix_web::web;

use crate::properties::application_layer::properties_service;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .route("/properties", web::get().to(properties_service::get_all))
            .route("/properties", web::post().to(properties_service::add)),
    );
}
