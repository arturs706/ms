use actix_web::{
    error, get, http::{
        header,
        Method,
    }, post, web, Error, HttpRequest, HttpResponse
};
use deadpool_redis::{Pool, Connection};
use futures_util::StreamExt as _;
use serde::{Deserialize, Serialize};
use serde_json;
use sqlx::FromRow;
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use url::Url;
use redis::AsyncCommands;
use dotenv::dotenv;



#[derive(Deserialize, Serialize, FromRow, Debug)]
struct User {
    user_id: String,
    name: String,
    username: String,
    mob_phone: String,
    acc_level: String,
    status: String,
    a_created: String,
}




#[get("/api/v1/users")]

pub async fn fetchusers(
        req: HttpRequest,
        method: Method,
        payload: web::Payload,
        redis_pool: web::Data<Pool>,
    ) -> Result<HttpResponse, Error> {
        let mut conn: Connection = match redis_pool.get().await {
            Ok(conn) => conn,
            Err(e) => {
                error::ErrorInternalServerError(e.to_string());
                return fetch_from_url(req, method, payload, &redis_pool).await;
            }
        };
    
        let users: Option<String> = match conn.get("users").await {
            Ok(users) => users,
            Err(e) => {
                error::ErrorInternalServerError(e.to_string());
                return fetch_from_url(req, method, payload, &redis_pool).await;
            }
        };
    
        match users {
            Some(users) => {
                let response = HttpResponse::Ok()
                    .content_type("application/json")
                    .body(users);
                Ok(response)
            }
            None => fetch_from_url(req, method, payload, &redis_pool).await,
        }
    }
    
    async fn fetch_from_url(
        req: HttpRequest,
        method: Method,
        mut payload: web::Payload,
        redis_pool: &web::Data<Pool>,

    ) -> Result<HttpResponse, Error> {
        dotenv().ok();
        let host = std::env::var("USER_SERVER_HOST").expect("USER_SERVER_HOST must be set");
        let port = std::env::var("USER_SERVER_PORT").expect("USER_SERVER_PORT must be set");
        let path = "/api/v1/users";

      
    
        let base_url = format!("http://{}:{}", host, port);
        let mut new_url = Url::parse(&base_url).unwrap();
        new_url.set_path(path);
        new_url.set_query(req.uri().query());
    
        let (tx, rx) = mpsc::unbounded_channel();
        actix_web::rt::spawn(async move {
            while let Some(chunk) = payload.next().await {
                tx.send(chunk).unwrap();
            }
        });
    
        let auth_header = req.headers().get(header::AUTHORIZATION).unwrap();
        let client = reqwest::Client::new();
        let mut init_req = client
            .request(method, new_url)
            .body(reqwest::Body::wrap_stream(UnboundedReceiverStream::new(rx)));
        init_req = init_req.header("Authorization", auth_header);
    
        let res = init_req
            .send()
            .await
            .map_err(error::ErrorInternalServerError)?;
    
        let bytes = res.bytes().await.map_err(error::ErrorInternalServerError)?;
        let users_vec: Vec<User> =
            serde_json::from_slice(&bytes).map_err(error::ErrorInternalServerError)?;
    
        let users_json =
            serde_json::to_string(&users_vec).map_err(error::ErrorInternalServerError)?;
            if let Some(mut connection) = redis_pool.get().await.ok() {
            let _: () = connection
                .set("users", users_json)
                .await
                .map_err(|_| error::ErrorInternalServerError("Internal Server Error"))?;
        }
    
        let response = HttpResponse::Ok()
            .content_type("application/json")
            .body(bytes);
        Ok(response)
    }
    

#[get("/api/v2/users")]
    pub async fn fetchuserstwo(
        req: HttpRequest,
        method: Method,
        mut payload: web::Payload,
    ) -> Result<HttpResponse, Error> {
        let host = std::env::var("USER_SERVER_HOST").expect("USER_SERVER_HOST must be set");
        let port = std::env::var("USER_SERVER_PORT").expect("USER_SERVER_PORT must be set");
        let path = "/api/v1/users";

      
        let base_url = format!("http://{}:{}", host, port);
    
        let mut new_url = Url::parse(&base_url).unwrap();
        new_url.set_path(path);
        new_url.set_query(req.uri().query());
    
        let (tx, rx) = mpsc::unbounded_channel();
        actix_web::rt::spawn(async move {
            while let Some(chunk) = payload.next().await {
                tx.send(chunk).map_err(|_ee| {
                }).ok();
            }
        });
    
        let auth_header = req.headers().get(header::AUTHORIZATION).ok_or_else(|| {
            error::ErrorUnauthorized("Authorization header is missing")
        })?;

        let client = reqwest::Client::new();
        let mut init_req = client
            .request(method, new_url)
            .body(reqwest::Body::wrap_stream(UnboundedReceiverStream::new(rx)));
            
            init_req = init_req.header(header::AUTHORIZATION, auth_header.clone());
    
        let res = init_req
            .send()
            .await
            .map_err(|e| {
                error::ErrorInternalServerError(e)
            })?;

   
        let bytes = res.bytes().await.map_err(|e| {
            error::ErrorInternalServerError(e)
        })?;
    
        let users_vec: Vec<User> = serde_json::from_slice(&bytes).map_err(|e| {
            error::ErrorInternalServerError(e)
        })?;
    
        let users_json = serde_json::to_string(&users_vec).map_err(|e| {
            error::ErrorInternalServerError(e)
        })?;
    
        let response = HttpResponse::Ok()
            .content_type("application/json")
            .body(users_json);
    
        Ok(response)
}
    


#[post("/api/v1/users/login")]
pub async fn login(
    req: HttpRequest,
    method: reqwest::Method,
    mut payload: web::Payload,
) -> Result<HttpResponse, Error> {
    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        body.extend_from_slice(&chunk);
    }
  
    let host = std::env::var("USER_SERVER_HOST").expect("USER_SERVER_HOST must be set");
    let port = std::env::var("USER_SERVER_PORT").expect("USER_SERVER_PORT must be set");
    let path = "/api/v1/users/login";
    let base_url = format!("http://{}:{}", host, port);
    let mut new_url = Url::parse(&base_url).unwrap();
    new_url.set_path(path);
    new_url.set_query(req.uri().query());

    let auth_header = req.headers().get(header::AUTHORIZATION).ok_or_else(|| {
        error::ErrorUnauthorized("Authorization header is missing")
    })?;

    let client = reqwest::Client::new();
    let init_req = client
        .request(method, new_url)
        .body(body.to_vec())
        .header(header::AUTHORIZATION, auth_header.clone())
        .header(header::CONTENT_TYPE, "application/json");

    let res = init_req
        .send()
        .await
        .map_err(|e| error::ErrorInternalServerError(e))?;

    let refresh_token = res.headers().get("refresh_token").ok_or_else(|| {
        error::ErrorUnauthorized("Refresh Token is missing")
    })?.to_str().map_err(|_| error::ErrorUnauthorized("Invalid Refresh Token"))?.to_string();

    let access_token = res.headers().get("access_token").ok_or_else(|| {
        error::ErrorUnauthorized("Access Token is missing")
    })?.to_str().map_err(|_| error::ErrorUnauthorized("Invalid Access Token"))?.to_string();

    let bytes = res.bytes().await.map_err(|e| error::ErrorInternalServerError(e))?;
    let json_value: serde_json::Value = serde_json::from_slice(&bytes).map_err(|e| error::ErrorInternalServerError(e))?;
    let json_str = serde_json::to_string(&json_value).map_err(|e| error::ErrorInternalServerError(e))?;

    let response = HttpResponse::Ok()
        .content_type("application/json")
        .append_header(("refresh_token", refresh_token))
        .append_header(("access_token", access_token))
        .body(json_str);
   
    Ok(response)
}


#[post("/api/v1/users")]
pub async fn register(
    req: HttpRequest,
    method: reqwest::Method,
    mut payload: web::Payload,
) -> Result<HttpResponse, Error> {
    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        body.extend_from_slice(&chunk);
    }
    let host = std::env::var("USER_SERVER_HOST").expect("USER_SERVER_HOST must be set");
    let port = std::env::var("USER_SERVER_PORT").expect("USER_SERVER_PORT must be set");
    let path = "/api/v1/users";
    let base_url = format!("http://{}:{}", host, port);

    let mut new_url = Url::parse(&base_url).unwrap();
    new_url.set_path(path);
    new_url.set_query(req.uri().query());

    let auth_header = req.headers().get(header::AUTHORIZATION).ok_or_else(|| {
        error::ErrorUnauthorized("Authorization header is missing")
    })?;

    let client = reqwest::Client::new();
    let init_req = client
        .request(method, new_url)
        .body(body.to_vec());

    let init_req = init_req.header(header::AUTHORIZATION, auth_header.clone()).header(header::CONTENT_TYPE, "application/json");

    let res = init_req
        .send()
        .await
        .map_err(|e| {
            error::ErrorInternalServerError(e)
        })?;

    let bytes = res.bytes().await.map_err(|e| {
        error::ErrorInternalServerError(e)
    })?;

     let json_value: serde_json::Value = serde_json::from_slice(&bytes).map_err(|e| {
        error::ErrorInternalServerError(e)
    })?;

    let json_str = serde_json::to_string(&json_value).map_err(|e| {
        error::ErrorInternalServerError(e)
    })?;

    let response = HttpResponse::Ok()
        .content_type("application/json")
        .body(json_str);
    
    Ok(response)
}
