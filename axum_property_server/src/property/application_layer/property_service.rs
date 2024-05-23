use axum::{
    extract::{Multipart, Path},
    http::{HeaderMap, StatusCode},
    Json,
};

use crate::property::{
    domain_layer::property::Property,
    infrastructure_layer::{
        custom_error_repo::CustomErrors, jwt_repo, property_repository::PropertyRepository,
    },
};
use axum_macros::debug_handler;
use crossbeam_channel::bounded;
use std::sync::{Arc, Mutex};
use tokio::fs as tfs;
use uuid::Uuid;

#[debug_handler]
pub async fn get_all_properties() -> Result<Json<Vec<Property>>, (StatusCode, String)> {
    let property_repository = PropertyRepository::new().await;
    match property_repository.get_all().await {
        Ok(properties) => Ok(Json(properties)),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
    }
}

#[debug_handler]
pub async fn get_property_by_id(
    Path(id): Path<Uuid>,
) -> Result<Json<Property>, (StatusCode, String)> {
    let property_repository = PropertyRepository::new().await;
    match property_repository.get_by_id(id).await {
        Ok(property) => Ok(Json(property)),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
    }
}

#[debug_handler]
pub async fn create_property(
    headers: HeaderMap,
    Json(property): Json<Property>,
) -> Result<Json<Property>, (StatusCode, String)> {
    let property_repository = PropertyRepository::new().await;
    let auth = headers
        .get("Authorization")
        .ok_or(CustomErrors::MissingCreds)
        .unwrap();
    let token = auth
        .to_str()
        .map_err(|_| CustomErrors::MissingCreds)
        .unwrap();
    let authtoken = token.replace("Bearer ", "");
    match jwt_repo::validate_token(&authtoken).await {
        Ok(_) => match property_repository.create(property).await {
            Ok(property) => Ok(Json(property)),
            Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
        },
        Err(e) => {
            println!("Access verify error: {:?}", e);
            match e.kind() {
                jsonwebtoken::errors::ErrorKind::InvalidToken => {
                    Err((StatusCode::UNAUTHORIZED, "Invalid Token".to_string()))
                }
                _ => Err((StatusCode::UNAUTHORIZED, "Invalid Key".to_string())),
            }
        }
    }
}

#[debug_handler]
pub async fn upload_images(
    Path(id): Path<Uuid>,
    headers: HeaderMap,
    mut payload: Multipart,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let auth = headers
        .get("Authorization")
        .ok_or(CustomErrors::MissingCreds)
        .unwrap();
    let token = auth
        .to_str()
        .map_err(|_| CustomErrors::MissingCreds)
        .unwrap();
    let authtoken = token.replace("Bearer ", "");
    match jwt_repo::validate_token(&authtoken).await {
        Ok(_) => {
            if let Err(error) = tfs::create_dir_all("./uploadedimg/").await {
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Error creating directory: {}", error),
                ));
            }
            let image_qty: usize = match headers.get("x-image-quantity") {
                Some(header_value) => header_value.to_str().unwrap_or("0").parse().unwrap(),
                None => "0".parse().unwrap(),
            };
            let mut image_url_vector: Vec<String> = Vec::with_capacity(image_qty as usize);
            let base_url: &str = "http://0.0.0.0:10003/api/v1/property/uploadedimg/";
            let start_time: std::time::Instant = std::time::Instant::now();
            let dir_spawn: &str = "./uploadedimg/";
            let (snd, rcv) = bounded(10);

            while let Some(mut field) = payload
                .next_field()
                .await
                .map_err(|error| (StatusCode::BAD_REQUEST, error.to_string()))?
            {   
                match field.content_type() {
                    Some("image/png") => "image/png".to_string(),
                    Some("image/jpeg") => "image/jpeg".to_string(),
                    Some("image/webp") => "image/webp".to_string(),
                    Some("image/avif") => "image/avif".to_string(),
                    _ => {
                        return Err((StatusCode::BAD_REQUEST, "Invalid content type".to_string()));
                    }
                };

                let name = field
                    .name()
                    .map(ToString::to_string)
                    .unwrap_or_else(|| "name".to_owned());

                let new_image_name: String = format!("{}-{}", name, Uuid::new_v4());
                image_url_vector.push(format!("{}{}.avif", base_url, new_image_name));
                let mut data = Vec::new();
                while let Some(chunk) = field
                    .chunk()
                    .await
                    .map_err(|error| (StatusCode::BAD_REQUEST, error.to_string()))?
                {
                    data.extend_from_slice(&chunk);
                }
                if data.len() > 10 * 1024 * 1024 { 
                    return Err((StatusCode::PAYLOAD_TOO_LARGE,format!("Image size exceeds 10 MB")));
                }

                let img_clone = Arc::new(Mutex::new(image::load_from_memory(&data).unwrap()));

                let snd_clone = snd.clone();
                let rcv_clone = rcv.clone();
                tokio::spawn(async move {
                    let resized_img = img_clone.lock().unwrap().resize(
                        1920,
                        1080,
                        image::imageops::FilterType::Triangle,
                    );
                    snd_clone.send(resized_img).unwrap();
                });
                tokio::spawn(async move {
                    let rcv_clone = rcv_clone.recv().unwrap();
                    let img_path = format!("{}{}.avif", dir_spawn, new_image_name);
                    let received_img_clone = rcv_clone.clone();
                    received_img_clone.save(img_path).unwrap();
                });
            }
            let elapsed_time = start_time.elapsed().as_millis();
            println!("{}", elapsed_time);
            let property_repository = PropertyRepository::new().await;
            match property_repository.save_images(id, image_url_vector).await {
                Ok(_) => Ok((StatusCode::OK, "Images uploaded successfully".to_string())),
                Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
            }
        }
        

        Err(e) => {
            println!("Access verify error: {:?}", e);
            match e.kind() {
                jsonwebtoken::errors::ErrorKind::InvalidToken => {
                    Err((StatusCode::UNAUTHORIZED, "Invalid Token".to_string()))
                }
                _ => Err((StatusCode::UNAUTHORIZED, "Invalid Key".to_string())),
            }
        }
    }
}
