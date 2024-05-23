#![allow(
    clippy::needless_borrow,
    clippy::needless_return,
    clippy::upper_case_acronyms,
    dead_code
)]
use actix_web::{web::Data, App, HttpServer};
use dotenv::dotenv;
use listenfd::ListenFd;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use properties::presentation_layer::property_photos_controller::configure_photos_routes;
use properties::infrastructure_layer::kafka_consumer::consume_and_print;
mod properties {
    pub mod application_layer;
    pub mod domain_layer;
    pub mod infrastructure_layer;
    pub mod presentation_layer;
}

#[derive(Clone)]
pub struct AppState {
    pub db: Pool<Postgres>,
    pub jwt_secret: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL_RO").expect("DATABASE_URL_RO must be set");
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let server_port = std::env::var("SERVER_PORT").expect("SERVER_PORT must be set");
    let server_host = std::env::var("SERVER_HOST").expect("SERVER_HOST must be set");
    let broker_url = std::env::var("SERVER_HOST").expect("SERVER_HOST must be set");

    let server_ip = format!("{}:{}", server_host, server_port);
    let pool = PgPoolOptions::new()
        .max_connections(1000)
        .connect(&database_url)
        .await
        .expect("Failed to create pool");
    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState {
                db: pool.clone(),
                jwt_secret: jwt_secret.clone(),
            }))
            .configure(configure_photos_routes)
        // .wrap(infrastructure_layer::auth_repo::Auth)
    });
    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => server.bind(server_ip)?,
    };
    let topics = "images";
    let brokers = broker_url.as_str();
    let group_id = "0";
  
    consume_and_print(brokers, group_id, topics).await;
    server.run().await.unwrap();
    Ok(())
}
