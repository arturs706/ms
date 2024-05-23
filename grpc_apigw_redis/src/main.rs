use self::merg_serv::MergServ;
use axum::middleware;
use axum::{
    error_handling::HandleErrorLayer,
    routing::get,
    BoxError, Router
};
mod errors;
use axum::{extract::ConnectInfo, ServiceExt};
use std::net::SocketAddr;
mod merg_serv;
use axum_client_ip::{InsecureClientIp, SecureClientIp, SecureClientIpSource};
use tower::{limit::ConcurrencyLimitLayer, ServiceBuilder};
use tower_governor::{errors::display_error, governor::GovernorConfigBuilder, GovernorLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use dotenv::dotenv;
mod rest_routes;
mod mware;
mod grpc_routes;
use crate::{grpc_routes::StaffUsersProxyService, staff_users::staff_users_server::StaffUsersServer};
pub mod staff_users {
    tonic::include_proto!("staffusers");
}
use tower_http::cors::{Any, CorsLayer};
use http::Method;
use deadpool_redis::{Config, Pool, Runtime};
use std::{env, time::Duration};




async fn web_root(
    insecure_ip: InsecureClientIp,
    secure_ip: SecureClientIp,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> String {
    format!(
        "{insecure_ip:?} {secure_ip:?} {addr}",
        insecure_ip = insecure_ip,
        secure_ip = secure_ip,
        addr = addr
    )
}
type DeadpoolPool = Pool;

#[derive(Clone)]
pub struct AppState {
    pub redis_pool: DeadpoolPool,
}

#[derive(Clone)]
pub struct RedisPool {
    pub pool: DeadpoolPool,
}


const MAX_POOL_SIZE: usize = 50;
const WAIT_TIMEOUT: Option<Duration> = Some(Duration::from_secs(10));

pub fn create_pool() -> Result<DeadpoolPool, String> {
    dotenv().ok();

    let redis_host = env::var("REDIS_HOST").unwrap_or_else(|_| "localhost".to_string());
    let redis_port = env::var("REDIS_PORT").unwrap_or_else(|_| "6379".to_string());
    let redis_password = env::var("REDIS_PASSWORD").ok();

    let redis_url = match redis_password {
        Some(password) => format!("redis://:{password}@{redis_host}:{redis_port}/"),
        None => format!("redis://{redis_host}:{redis_port}/"),
    };

    let config = Config::from_url(redis_url);

    config
        .builder()
        .map(|b| {
            b.max_size(MAX_POOL_SIZE)
                .wait_timeout(WAIT_TIMEOUT)
                // TODO needs create_timeout/recycle timeout?
                .runtime(Runtime::Tokio1)
                .build()
                .unwrap() // TODO don't panic. flat_map can't be used???
        })
        .map_err(|e| e.to_string())
}



fn main_app() -> Router {
        let governor_conf = Box::new(
            GovernorConfigBuilder::default()
                .per_second(2)
                .burst_size(5)
                .finish()
                .unwrap(),
        );
        let concurrency_limi_layer = ConcurrencyLimitLayer::new(10);
        let cors = CorsLayer::new()
        .allow_headers(Any)
        .allow_methods([Method::POST, Method::GET, Method::OPTIONS, Method::PUT, Method::DELETE])
        .allow_origin(["http://localhost:2000".parse().unwrap(), "http://localhost:2001".parse().unwrap(), "http://localhost:2002".parse().unwrap()]);
        // .allow_origin(Any);
        Router::new()
            .route("/", get(web_root))
            .route("/api/v1/users", get(rest_routes::fetchusershandler))
            .layer(SecureClientIpSource::ConnectInfo.into_extension())
            .layer(concurrency_limi_layer)
            .layer(
                ServiceBuilder::new()
                    .layer(HandleErrorLayer::new(|e: BoxError| async move {
                        display_error(e)
                    }))
                    .layer(GovernorLayer {
                        config: Box::leak(governor_conf),
                    }),
            )            
            .layer(middleware::from_fn(mware::add_token))
            .layer(cors)
            
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "debuglogs".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();



    async fn start_api_gw() {
    
        let us_prox: StaffUsersProxyService = StaffUsersProxyService::default();
        let grpc = StaffUsersServer::new(us_prox);
        let rest = main_app();
        let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
        let service = MergServ::new(rest, grpc);
        println!("Server running on http://{}", addr);

        let _ = axum::Server::bind(&addr)
        .serve(service
            .into_make_service_with_connect_info::<SocketAddr>()
        
        )
        .await;
    }

    start_api_gw().await;

}
