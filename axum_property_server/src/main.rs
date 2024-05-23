use axum::{
    http::{
        header::{ACCEPT, AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE},
        HeaderName, HeaderValue, Method,
    },
    middleware,
};
use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::thread;
use std::time::Instant;
use tower_http::cors::CorsLayer;
mod property {
    pub mod application_layer;
    pub mod domain_layer;
    pub mod infrastructure_layer;
    pub mod presentation_layer;
}
use property::{
    infrastructure_layer::route_auth_repo::auth_middleware,
    presentation_layer::property_controller::create_router,
};

#[derive(Clone, Debug)]
pub struct AppState {
    pub database: Database,
    pub jwt_secret: JWTToken,
}
#[derive(Clone, Debug)]
pub struct Database {
    pub db: Pool<Postgres>,
}

#[derive(Clone, Debug)]
pub struct JWTToken {
    pub jwt_secret: String,
}

impl AppState {
    pub async fn new() -> Self {
        let database_url: String = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let jwt_secret: String = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await
            .expect("Failed to create pool");

        AppState {
            database: Database { db: pool },
            jwt_secret: JWTToken { jwt_secret },
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let x_custom_header = HeaderName::from_static("x-image-quantity");
    tracing_subscriber::fmt::init();

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([
            AUTHORIZATION,
            ACCEPT,
            CONTENT_TYPE,
            CONTENT_LENGTH,
            x_custom_header,
        ]);

    let app = create_router().layer(cors);
    // .route_layer(middleware::from_fn(auth_middleware));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:10003")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
