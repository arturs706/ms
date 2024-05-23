use actix_web::web;

use crate::landlords::application_layer::landlords_service;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .route("/landlords", web::get().to(landlords_service::get_all))
            .route("/landlords", web::post().to(landlords_service::add))
            .route("/landlords/{landlord_id}/address", web::post().to(landlords_service::add_address))
            .route("/landlords/{landlord_id}", web::delete().to(landlords_service::delete_landlord))

    );
}
