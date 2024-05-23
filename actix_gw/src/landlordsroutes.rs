use actix_web::{
    error, get, http::{
        header,
        Method,
    }, post, web, Error, HttpRequest, HttpResponse
};
use chrono::{NaiveDate, NaiveDateTime};
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
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "landlord_status_type")]
pub enum LandlordStatus {
    ACTIVE,
    INACTIVE,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Landlord {
    pub landlord_id: Option<Uuid>,
    pub title: Option<String>,
    pub name: Option<String>,
    pub surname: Option<String>,
    pub company: Option<String>,
    pub phone: String,
    pub email: String,
    pub status: Option<LandlordStatus>,
    pub donotdelete: Option<NaiveDate>,
    pub createdby: Option<String>,
    pub createdat: Option<NaiveDateTime>
}



#[get("/api/v1/landlords")]

pub async fn fetchlandlords(
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
    
        let landlords: Option<String> = match conn.get("landlords").await {
            Ok(landlords) => landlords,
            Err(e) => {
                error::ErrorInternalServerError(e.to_string());
                return fetch_from_url(req, method, payload, &redis_pool).await;
            }
        };
    
        match landlords {
            Some(landlords) => {
                let response = HttpResponse::Ok()
                    .content_type("application/json")
                    .body(landlords);
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
        let host = std::env::var("LANDLORDS_SERVER_HOST").expect("LANDLORDS_SERVER_HOST must be set");
        let port = std::env::var("LANDLORDS_SERVER_PORT").expect("LANDLORDS_SERVER_PORT must be set");
        let path = "/api/v1/landlords";
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
        let landlords_vec: Vec<Landlord> =
            serde_json::from_slice(&bytes).map_err(error::ErrorInternalServerError)?;
    
        let landlords_json =
            serde_json::to_string(&landlords_vec).map_err(error::ErrorInternalServerError)?;
            if let Some(mut connection) = redis_pool.get().await.ok() {
            let _: () = connection
                .set("landlords", landlords_json)
                .await
                .map_err(|_| error::ErrorInternalServerError("Internal Server Error"))?;
        }
    
        let response = HttpResponse::Ok()
            .content_type("application/json")
            .body(bytes);
        Ok(response)
    }
    

#[get("/api/v2/landlords")]
    pub async fn fetchlandlordstwo(
        req: HttpRequest,
        method: Method,
        mut payload: web::Payload,
    ) -> Result<HttpResponse, Error> {
        let host = std::env::var("LANDLORDS_SERVER_HOST").expect("LANDLORDS_SERVER_HOST must be set");
        let port = std::env::var("LANDLORDS_SERVER_PORT").expect("LANDLORDS_SERVER_PORT must be set");
        let path = "/api/v1/landlords";

      
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
    
        let landlords_vec: Vec<Landlord> = serde_json::from_slice(&bytes).map_err(|e| {
            error::ErrorInternalServerError(e)
        })?;
    
        let landlords_json = serde_json::to_string(&landlords_vec).map_err(|e| {
            error::ErrorInternalServerError(e)
        })?;
    
        let response = HttpResponse::Ok()
            .content_type("application/json")
            .body(landlords_json);
    
        Ok(response)
}
    



#[post("/api/v1/landlords")]
pub async fn registerlandlord(
    req: HttpRequest,
    method: reqwest::Method,
    mut payload: web::Payload,
) -> Result<HttpResponse, Error> {
    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        body.extend_from_slice(&chunk);
    }
    let host = std::env::var("LANDLORDS_SERVER_HOST").expect("LANDLORDS_SERVER_HOST must be set");
    let port = std::env::var("LANDLORDS_SERVER_PORT").expect("LANDLORDS_SERVER_PORT must be set");
    let path = "/api/v1/landlords";
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



#[post("/api/v1/landlords/{landlord_id}/address")]
pub async fn registerlandlordaddress(
    req: HttpRequest,
    method: reqwest::Method,
    mut payload: web::Payload,
) -> Result<HttpResponse, Error> {
    println!("registerlandlordaddress");
    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        body.extend_from_slice(&chunk);
    }
    let host = std::env::var("LANDLORDS_SERVER_HOST").expect("LANDLORDS_SERVER_HOST must be set");
    let port = std::env::var("LANDLORDS_SERVER_PORT").expect("LANDLORDS_SERVER_PORT must be set");
    let path = format!("/api/v1/landlords/{}/address", req.match_info().get("landlord_id").unwrap());
    let base_url = format!("http://{}:{}", host, port);

   
    let mut new_url = Url::parse(&base_url).unwrap();
    new_url.set_path(&path);
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
