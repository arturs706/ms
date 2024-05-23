use actix_web::web;

use crate::properties::application_layer::properties_service;

pub fn configure_address_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .route(
                "/properties/address",
                web::get().to(properties_service::get_all),
            )
            .route(
                "/properties/address/{property_id}",
                web::post().to(properties_service::add),
            ),
    );
}
