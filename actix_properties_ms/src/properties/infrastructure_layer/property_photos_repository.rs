use crate::{properties::domain_layer::property_photos::PropertyPhotos, AppState};
use actix_web::web::Json;
use actix_web::HttpResponse;
use actix_web::{http::StatusCode, HttpRequest};
use crossbeam_channel::bounded;
use futures_util::StreamExt;
use futures_util::TryStreamExt as _;
use image::codecs::avif;
use image::codecs::png::FilterType;
use image::DynamicImage;
use mime::{self, Mime, IMAGE, IMAGE_BMP, IMAGE_JPEG, IMAGE_PNG, IMAGE_SVG};
use serde_json::json;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::Mutex;
use tokio::fs as tfs;
use tokio::io::AsyncWriteExt as _;
use uuid::Uuid;
use dotenv::dotenv;

pub struct PropertyPhotosRepository {}

impl PropertyPhotosRepository {
    pub fn new() -> Self {
        PropertyPhotosRepository {}
    }

    pub async fn get_all(&self, state: Arc<AppState>) -> Result<Vec<PropertyPhotos>, Json<String>> {
        let result = sqlx::query_as::<_, PropertyPhotos>("SELECT * FROM property_photos")
            .fetch_all(&state.db)
            .await;

        match result {
            Ok(property_photos) => Ok(property_photos),
            Err(e) => Err(Json(e.to_string())),
        }
    }

    pub async fn save(
        &self,
        state: Arc<AppState>,
        property_photos: PropertyPhotos,
    ) -> Result<serde_json::Value, Json<String>> {
        let result = sqlx::query_as::<_, PropertyPhotos>(
            r#"
            INSERT INTO property_photos (property_photos_id, property_id, photo_urls)
            VALUES ($1, $2, $3)
            RETURNING *
            "#,
        )
        .bind(&property_photos.property_photos_id)
        .bind(&property_photos.property_id)
        .bind(&property_photos.photo_urls)
        .fetch_one(&state.db)
        .await;

        match result {
            Ok(property_photos) => {
                let json_with_with_message: serde_json::Value =
                    json!({ "message": "success", "address": property_photos });
                Ok(json_with_with_message)
            }
            Err(e) => Err(Json(e.to_string())),
        }
    }

    pub async fn get_by_id(
        &self,
        state: Arc<AppState>,
        property_id: Uuid,
    ) -> Result<Vec<PropertyPhotos>, Json<String>> {
        let result = sqlx::query_as::<_, PropertyPhotos>(
            "SELECT * FROM property_photos WHERE property_id = $1",
        )
        .bind(&property_id)
        .fetch_all(&state.db)
        .await;

        match result {
            Ok(property_photos) => Ok(property_photos),
            Err(e) => Err(Json(e.to_string())),
        }
    }


    pub async fn upload_images(
        &self,
        state: Arc<AppState>,
        property_id: Uuid,
        payload: &mut actix_multipart::Multipart,
        req: HttpRequest,
    ) -> Result<(StatusCode, Json<serde_json::Value>), (StatusCode, Json<serde_json::Value>)> {
        let content_length: usize = match req.headers().get("content-length") {
            Some(header_value) => header_value.to_str().unwrap_or("0").parse().unwrap(),
            None => "0".parse().unwrap(),
        };
        
        let max_file_count: usize = 10;
        let max_file_size: usize = 10_000_000;
        let avif: mime::Mime = "image/avif".parse().unwrap();
        let webp: mime::Mime = "image/webp".parse().unwrap();
        let legal_filetypes: Vec<Mime> = vec![IMAGE_PNG, IMAGE_JPEG, IMAGE_BMP, IMAGE_SVG, avif, webp];
        let mut image_url_vector: Vec<String> = Vec::with_capacity(10 as usize);
        let mut current_count: usize = 0;
        let dir: &str = "./data/";
    
        if !Path::new(dir).exists() {
            fs::create_dir_all(dir).expect("Failed to create directory");
        }
    
        if content_length > max_file_size {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "error": "Payload Too Large",
                    "message": "The payload is too large",
                })),
            ));
        }
    
        while let Ok(Some(mut field)) = payload.try_next().await {
            if current_count >= max_file_count {
                break;
            }
    
            let filetype: Option<&Mime> = field.content_type();
            if filetype.is_none() {
                continue;
            }
    
            if !legal_filetypes.contains(&filetype.unwrap()) {
                continue;
            }
    
            let destination: String = format!(
                "{}{}-{}",
                dir,
                Uuid::new_v4(),
                field.content_disposition().get_filename().unwrap()
            );
    
            let (snd, rcv) = bounded(10);
            let img_clone = Arc::new(Mutex::new(
                image::load_from_memory(&field.next().await.unwrap().unwrap()).unwrap(),
            ));
            let snd_clone = snd.clone();
            let rcv_clone = rcv.clone();
    
            tokio::spawn(async move {
                let resized_img = img_clone.lock();
                match resized_img {
                    Ok(img) => {
                        let img = img.clone();
                        let img = img.resize(1920, 1080, image::imageops::FilterType::Triangle);
                        snd_clone.send(img).unwrap();
                    }
                    Err(e) => {
                        println!("Error: {:#?}", e);
                    }
                }
            });
    
            tokio::spawn(async move {
                match rcv_clone.recv() {
                    Ok(rcv_clone) => {
                        let server_port = std::env::var("SERVER_PORT").expect("SERVER_PORT must be set");
                        let server_host = std::env::var("SERVER_HOST").expect("SERVER_HOST must be set");
                        let img_path = remove_extension(&Path::new(&destination))
                            .to_string_lossy()
                            .to_string()
                            + ".avif";
                        let img_path_cleaned = remove_dot_slash(&Path::new(&img_path))
                            .to_string_lossy()
                            .to_string();
                        let full_image_path = format!("http://{}:{}/{}", server_host, server_port, img_path_cleaned);
                        match rcv_clone.save(&img_path) {
                            Ok(_) => println!("Image saved successfully to {}", full_image_path),
                            Err(e) => println!("Failed to save image: {:#?}", e),
                        }
                    }
                    Err(e) => {
                        println!("Error: {:#?}", e);
                    }
                }
            });
        }
    
        Ok((
            StatusCode::OK,
            Json(json!({
                "message": "success",
                "address": "Images uploaded successfully",
            })),
        ))
    }
}

fn remove_extension(path: &Path) -> PathBuf {
    let mut path_buf = PathBuf::from(path);
    path_buf.set_extension("");
    path_buf
}

fn remove_dot_slash(path: &Path) -> PathBuf {
    let path_str = path.to_string_lossy().to_string();
    PathBuf::from(path_str.trim_start_matches("./"))
}