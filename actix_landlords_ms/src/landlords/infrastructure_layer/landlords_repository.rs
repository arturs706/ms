#![allow(dead_code)]

use actix_web::web::Json;
use serde_json::json;
use uuid::Uuid;
use std::sync::Arc;
use crate::{landlords::domain_layer::{landlord::{self, Landlord, LandlordStatus}, landlord_address::LandlordAddress}, AppState};


pub struct LandlordsRepository {}

impl LandlordsRepository {
    pub fn new() -> Self {
        LandlordsRepository {}
    }
    pub async fn get_all(&self, state: Arc<AppState>) -> Result<Vec<Landlord>, Json<String>> {
        let result = sqlx::query_as::<_, Landlord>("SELECT * FROM llgendetails")
            .fetch_all(&state.db)
            .await;
        match result {
            Ok(landlords) => Ok(landlords),
            Err(e) => Err(Json(e.to_string())),
        }
    }


    
    pub async fn save(&self, state: Arc<AppState>, landlord: Landlord) -> Result<serde_json::Value, Json<serde_json::Value>>  {
        let mut landlord_id = Uuid::new_v4();
        if landlord.landlord_id != None {
            landlord_id = landlord.landlord_id.unwrap();
        }
        let created_at = chrono::Utc::now();
        let mut donotdelete_date = None;
        if Some(landlord.donotdelete) != None {
            donotdelete_date = Some(landlord.donotdelete);
        }
        let record = sqlx::query_as::<_, Landlord>(
            "INSERT INTO llgendetails (landlord_id, title, name, surname, company, phone, email, status, donotdelete, createdby, createdat)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11) RETURNING *"
        )
        .bind(landlord_id)
        .bind(landlord.title)
        .bind(landlord.name)
        .bind(landlord.surname)
        .bind(landlord.company)
        .bind(landlord.phone)
        .bind(landlord.email)
        .bind(LandlordStatus::ACTIVE)
        .bind(donotdelete_date)
        .bind(landlord.createdby)
        .bind(created_at)
        .fetch_one(&state.db)
        .await;
        match record {
            Ok(landlord) => {
                let json_with_with_message: serde_json::Value = json!({ "message": "success", "landlord": landlord });
                Ok(json_with_with_message)
            }
            Err(e) => match e {
                sqlx::Error::Database(e) => {
                    if let Some(pg_error) = e.constraint() {
                        if pg_error.contains("llgendetails_phone_key") {
                            Err(Json(json!({ "error": "Duplicate key error", "message": "Phone number is already taken" })))
                        } else if pg_error.contains("llgendetails_email_key") {
                            Err(Json(json!({ "error": "Duplicate key error", "message": "Email is already taken" })))
                        } else {
                            Err(Json(serde_json::json!({ "error": e.to_string() })))
                        }
                    } else {
                        Err(Json(serde_json::json!({ "error": e.to_string() })))
                    }
                }
                _ => Err(Json(serde_json::json!({ "error": e.to_string() }))),
            },
        }
    }




    pub async fn saveaddress(&self, state: Arc<AppState>, landlord_id: Uuid, landlord_addr: LandlordAddress) -> Result<serde_json::Value, Json<String>> {
        println!("saveaddress");
        let lladdr_id = Uuid::new_v4();
        let result = sqlx::query_as::<_, LandlordAddress>(
            r#"
            INSERT INTO lladdr (lladdr_id, landlord_id, addrone, addrtwo, towncity, county, postcode, country)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING *
            "#,
        )
        .bind(lladdr_id)
        .bind(landlord_id)
        .bind(landlord_addr.addrone)
        .bind(landlord_addr.addrtwo)
        .bind(landlord_addr.towncity)
        .bind(landlord_addr.county)
        .bind(landlord_addr.postcode)
        .bind(landlord_addr.country)
        .fetch_one(&state.db)
        .await;
        match result {
            Ok(landlord_addr) => {
                let json_with_with_message: serde_json::Value = json!({ "message": "success", "address": landlord_addr });
                Ok(json_with_with_message)
            }
            Err(e) => Err(Json(e.to_string())),
        }
    
    }

    pub async fn delete_landlord(&self, state: Arc<AppState>, landlord_id: Uuid) -> Result<Landlord, Json<String>> {
        let result = sqlx::query_as::<_, Landlord>(
            "DELETE FROM llgendetails WHERE landlord_id = $1 RETURNING *"
        )
        .bind(landlord_id)
        .fetch_one(&state.db)
        .await;
        match result {
            Ok(landlord) => Ok(landlord),
            Err(e) => Err(Json(e.to_string())),
        }
    }
}