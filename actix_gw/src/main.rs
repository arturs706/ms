#![allow(
    clippy::match_result_ok,
    clippy::needless_return,
    clippy::single_component_path_imports
)]
use actix_cors::Cors;
use actix_web::{web::Data, App, HttpServer};
use deadpool_redis::{Config, Pool, Runtime};
use dotenv::dotenv;
use listenfd::ListenFd;
use std::time::Duration;
mod errors;
mod midw;
mod userroutes;
mod landlordsroutes;


pub struct AppState {
    pub base_url: Vec<String>,
}

type DeadpoolPool = Pool;

const MAX_POOL_SIZE: usize = 50;
const WAIT_TIMEOUT: Option<Duration> = Some(Duration::from_secs(10));

pub fn create_pool() -> Result<DeadpoolPool, String> {
    dotenv().ok();
    let redis_host: String = std::env::var("REDIS_HOST").expect("REDIS_HOST must be set");
    let redis_port: String = std::env::var("REDIS_PORT").expect("REDIS_PORT must be set");
    let redis_url = format!("redis://{}:{}", redis_host, redis_port);
    let config = Config::from_url(redis_url);
    config
        .builder()
        .map(|b| {
            b.max_size(MAX_POOL_SIZE)
                .wait_timeout(WAIT_TIMEOUT)
                .runtime(Runtime::Tokio1)
                .build()
                .unwrap()
        })
        .map_err(|e| e.to_string())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let pool = match create_pool() {
        Ok(pool) => pool,
        Err(err) => {
            eprintln!("Error creating Redis pool: {}", err);
            return Ok(());
        }
    };
    let mut listenfd = ListenFd::from_env();
    let redis_pool_data: Data<_> = actix_web::web::Data::new(pool);

    let mut server = HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("https://artdevldn.co.uk")
            .allowed_origin("https://www.artdevldn.co.uk")
            .allowed_origin("frontend-app.frontend.svc.cluster.local:4000")
            .allowed_origin("https://frontend-app.frontend.svc.cluster.local:4000")
            .allowed_origin("http://frontend-app.frontend.svc.cluster.local:4000")
            .allowed_origin("http://localhost:4000")
            .allowed_origin_fn(|origin, _req_head| origin.as_bytes().ends_with(b".rust-lang.org"))
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .service(userroutes::fetchusers)
            .service(userroutes::fetchuserstwo)
            .service(userroutes::login)
            .service(userroutes::register)
            .service(landlordsroutes::fetchlandlords)
            .service(landlordsroutes::fetchlandlordstwo)
            .service(landlordsroutes::registerlandlord)
            .service(landlordsroutes::registerlandlordaddress)
            .app_data(redis_pool_data.clone())
            .wrap(cors)
            .wrap(midw::AddToken)
    });
    let port =
        std::env::var("USER_SERVER_PORT_GW").expect("Please set USER_SERVER_PORT_GW in .env");

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => server.bind(format!("{}:{}", "0.0.0.0", port))?,
    };
    server.run().await
}
