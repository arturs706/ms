
use std::sync::Arc;
use std::{env, time::Duration};
use deadpool_redis::redis::AsyncCommands;
use deadpool_redis::{Config, Connection, Pool, Runtime};
use tonic::metadata::MetadataMap;
use tonic::transport::Endpoint;
use staff_users::staff_users_server::StaffUsers;
use staff_users::staff_users_client::StaffUsersClient;
use staff_users::{RegisterUserRequest, RegisterUserResponse, AllUserRetrieve, EmptyReqRes};
use tonic::{Response, Status};
use crate::staff_users::{self, SingleUserRetrieve};
use crate::mware::{self, add_token_to_request};
use dotenv::dotenv;
use serde_json::{json, Value};

// rpc GetAllUsers (Empty) returns (AllUserRetrieve) {}


#[derive(Debug, Default)]
pub struct StaffUsersProxyService {}

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
    
    let redis_host = env::var("REDIS_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
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
                .runtime(Runtime::Tokio1)
                .build()
                .unwrap() // TODO don't panic. flat_map can't be used???
        })
        .map_err(|e| e.to_string())
}



#[tonic::async_trait]
impl StaffUsers for StaffUsersProxyService {
    async fn register_user(
        &self,
        request: tonic::Request<RegisterUserRequest>,
    ) -> Result<Response<RegisterUserResponse>, Status> {
        println!("Received request: {:?}", request);
        dotenv::dotenv().ok();
        let user_server_host = std::env::var("USER_SERVER_HOST").expect("USER_SERVER_HOST must be set");
        let user_server_port = std::env::var("USER_SERVER_PORT").expect("USER_SERVER_PORT must be set");
        let user_server_url = Arc::new(format!("http://{}:{}", user_server_host, user_server_port));
        let user_server_url_clone = Arc::clone(&user_server_url);
        let user_server_url_str = user_server_url_clone.as_ref().to_string();
        let user_server_url_ref: &'static str = Box::leak(user_server_url_str.into_boxed_str());
        let channel: tonic::transport::Channel = Endpoint::from_static(user_server_url_ref)
            .connect()
            .await
            .unwrap();
        let mut client = StaffUsersClient::new(channel);
        let response = client.register_user(request).await?;
        Ok(Response::new(response.into_inner()))
    }

    async fn get_all_users(
        &self,
        _request: tonic::Request<EmptyReqRes>,
    ) -> Result<Response<AllUserRetrieve>, Status> {
        let redis_pool = create_pool().unwrap();
        let mut conn: Connection = match redis_pool.get().await {
            Ok(conn) => conn,
            Err(e) => { return Err(Status::internal(e.to_string())); }
        };
        let users: Option<String> = match conn.get("users").await {
            Ok(users) => users,
            Err(e) => { return Err(Status::internal(e.to_string())); }
        };

        match users {
            Some(users_retrieved) => {
                println!("Users retrieved from Redis");
                let users: Vec<SingleUserRetrieve> = match serde_json::from_str(&users_retrieved) {
                    Ok(Value::Array(users_json)) => users_json.into_iter()
                        .map(|user_json| {
                            let user_id = user_json.get("user_id").unwrap().as_str().unwrap().to_string();
                            let name = user_json.get("name").unwrap().as_str().unwrap().to_string();
                            let username = user_json.get("username").unwrap().as_str().unwrap().to_string();
                            let mob_phone = user_json.get("mob_phone").unwrap().as_str().unwrap().to_string();
                            let acc_level = user_json.get("acc_level").unwrap().as_str().unwrap().to_string();
                            let status = user_json.get("status").unwrap().as_str().unwrap().to_string();
                            let a_created = user_json.get("a_created").unwrap().as_str().unwrap().to_string();
                            SingleUserRetrieve { user_id, name, username, mob_phone, acc_level, status, a_created }
                        })
                        .collect(),
                    _ => vec![],
                };
                let users = AllUserRetrieve { users };
                println!("users: {:?}", users);
                Ok(Response::new(users))
            }
            None => {
                println!("Users not found in Redis");
                dotenv::dotenv().ok();
                let user_server_host = std::env::var("USER_SERVER_HOST").expect("USER_SERVER_HOST must be set");
                let user_server_port = std::env::var("USER_SERVER_PORT").expect("USER_SERVER_PORT must be set");
                let user_server_url = Arc::new(format!("http://{}:{}", user_server_host, user_server_port));
                let user_server_url_clone = Arc::clone(&user_server_url);
                let user_server_url_str = user_server_url_clone.as_ref().to_string();
                let user_server_url_ref: &'static str = Box::leak(user_server_url_str.into_boxed_str());
                let channel: tonic::transport::Channel = Endpoint::from_static(user_server_url_ref)
                    .connect()
                    .await
                    .unwrap();
                let mut request = tonic::Request::new(EmptyReqRes { message: Some("".to_string()) });
                request = add_token_to_request(request);
                let mut client = StaffUsersClient::new(channel);
                let response: Response<AllUserRetrieve> = client.get_all_users(request).await?;
                // save to Redis
                let users = response.into_inner();
                let users_json: Vec<Value> = users.users.iter().map(|user| {
                    json!({
                        "user_id": &user.user_id,
                        "name": &user.name,
                        "username": &user.username,
                        "mob_phone": &user.mob_phone,
                        "acc_level": &user.acc_level,
                        "status": &user.status,
                        "a_created": &user.a_created,
                    })
                }).collect();
                if let Some(mut connection) = redis_pool.get().await.ok() {
                    let users_json_string = serde_json::to_string(&users_json).expect("Error serializing users to JSON");
                    let _: () = connection
                        .set("users", users_json_string)
                        .await
                        .expect("Error setting users in Redis");
                }
                println!("users: {:?}", users);
                Ok(Response::new(users))
                // save to Redis

            }
        }
    }





    async fn get_all_users_no_redis(
        &self,
        _request: tonic::Request<EmptyReqRes>,
    ) -> Result<Response<AllUserRetrieve>, Status> {
        dotenv::dotenv().ok();
        let user_server_host = std::env::var("USER_SERVER_HOST").expect("USER_SERVER_HOST must be set");
        let user_server_port = std::env::var("USER_SERVER_PORT").expect("USER_SERVER_PORT must be set");
        let user_server_url = Arc::new(format!("http://{}:{}", user_server_host, user_server_port));
        let user_server_url_clone = Arc::clone(&user_server_url);
        let user_server_url_str = user_server_url_clone.as_ref().to_string();
        let user_server_url_ref: &'static str = Box::leak(user_server_url_str.into_boxed_str());
        println!("user_server_url_ref: {:?}", user_server_url_ref);
        let channel: tonic::transport::Channel = Endpoint::from_static(user_server_url_ref)
            .connect()
            .await
            .unwrap();
        let mut request = tonic::Request::new(EmptyReqRes { message: Some("".to_string()) });
        request = add_token_to_request(request);
        let mut client = StaffUsersClient::new(channel);
        let response = client.get_all_users(request).await?;
        println!("response: {:?}", response);
        Ok(Response::new(response.into_inner()))
    }
}

