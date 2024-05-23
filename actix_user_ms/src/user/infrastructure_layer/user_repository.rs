#![allow(dead_code)]

use actix_web::{error::ResponseError, web::Json};
use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};
use chrono::NaiveDateTime;
use derive_more::Display;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::prelude::FromRow;
use uuid::Uuid;
use std::sync::Arc;
use crate::{user::domain_layer::user::StaffUser, AppState};

#[derive(Debug, Display, Serialize)]
pub enum CustomErrors {
    #[display(fmt = "Database error: {}", _0)]
    DatabaseError(String),
    #[display(fmt = "Not authorized")]
    NotAuthorized,
    
}

impl ResponseError for CustomErrors {}

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct StaffUserDB {
    pub user_id: Option<Uuid>,
    pub name: Option<String>,
    pub username: String,
    pub mob_phone: Option<String>,
    pub passwd: String,
    pub acc_level: Option<String>,
    pub status: Option<String>,
    pub a_created: Option<NaiveDateTime>,
}

pub struct UserRepository {}

impl UserRepository {
    pub fn new() -> Self {
        UserRepository {}
    }

    pub async fn get_all(&self, state: Arc<AppState>) -> Result<Vec<StaffUserDB>, CustomErrors> {
        let records = sqlx::query_as::<_, StaffUserDB>("SELECT * FROM staff_users")
            .fetch_all(&state.db)
            .await;

        match records {
            Ok(users) => Ok(users),
            Err(e) => Err(CustomErrors::DatabaseError(e.to_string())),
        }
    }

    pub async fn get_by_id(&self, state: Arc<AppState>, user_id: Uuid) -> Result<StaffUserDB, CustomErrors> {
        let record = sqlx::query_as::<_, StaffUserDB>("SELECT * FROM staff_users WHERE user_id = $1")
            .bind(user_id)
            .fetch_one(&state.db)
            .await;
        match record {
            Ok(user) => Ok(user),
            Err(e) => Err(CustomErrors::DatabaseError(e.to_string())),
        }
    }

    pub async fn save(&self, state: Arc<AppState>, user: StaffUser) -> Result<StaffUserDB, Json<serde_json::Value>> {
        let user_id = Uuid::new_v4();
        let acc_level = user.acc_level.unwrap_or_else(|| "trainee".to_string());
        let status = user.status.unwrap_or_else(|| "active".to_string());
        let a_created = user.a_created.unwrap_or_else(|| NaiveDateTime::from_timestamp(0, 0));
        let argon2 = Argon2::default();
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = argon2
            .hash_password(user.passwd.as_bytes(), &salt)
            .map_err(|e| serde_json::Value::String(e.to_string()))
            .unwrap();
    
        let record = sqlx::query_as::<_, StaffUserDB>(
            "INSERT INTO staff_users (user_id, name, username, mob_phone, passwd, acc_level, status, a_created) VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING *",
        )
        .bind(user_id)
        .bind(user.name)
        .bind(user.username)
        .bind(user.mob_phone)
        .bind(password_hash.to_string())
        .bind(acc_level)
        .bind(status)
        .bind(a_created)
        .fetch_one(&state.db)
        .await;
    
        match record {
            Ok(user) => Ok(user),
            Err(e) => match e {
                sqlx::Error::Database(e) => {
                    if let Some(pg_error) = e.constraint() {
                        if pg_error.contains("idx_username") {
                            Err(Json(json!({ "error": "Duplicate key error", "message": "Username already exists" })))
                        } else if pg_error.contains("idx_mob_phone") {
                            Err(Json(json!({ "error": "Duplicate key error", "message": "Mobile phone number already exists" })))
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




    pub async fn update(&self, state: Arc<AppState>, user: StaffUser) -> Result<StaffUserDB, CustomErrors> {
        let record = sqlx::query_as::<_, StaffUserDB>(
            "UPDATE staff_users SET name = $1, username = $2, mob_phone = $3, passwd = $4, acc_level = $5, status = $6, a_created = $7 WHERE user_id = $8 RETURNING *",
        )
        .bind(user.name)
        .bind(user.username)
        .bind(user.mob_phone)
        .bind(user.passwd)
        .bind(user.acc_level)
        .bind(user.status)
        .bind(user.a_created)
        .bind(user.user_id)
        .fetch_one(&state.db)
        .await;
        match record {
            Ok(user) => Ok(user),
            Err(e) => Err(CustomErrors::DatabaseError(e.to_string())),
        }
    }

    pub async fn delete(&self, state: Arc<AppState>, user_id: Uuid) -> Result<(), CustomErrors> {
        let record = sqlx::query("DELETE FROM staff_users WHERE user_id = $1")
            .bind(user_id)
            .execute(&state.db)
            .await;
        match record {
            Ok(_) => Ok(()),
            Err(e) => Err(CustomErrors::DatabaseError(e.to_string())),
        }
    }

    pub async fn login(&self, state: Arc<AppState>, user: StaffUser) -> Result<StaffUserDB, CustomErrors> {
        let record = sqlx::query_as::<_, StaffUserDB>(
            "SELECT * FROM staff_users WHERE username = $1",
        )
        .bind(&user.username)
        .fetch_one(&state.db)
        .await;
        match record {
            Ok(user_db) => {
                let parsed_hash = PasswordHash::new(&user_db.passwd).unwrap();
                let is_pass_valid = Argon2::default()
                    .verify_password(user.passwd.as_bytes(), &parsed_hash)
                    .is_ok();
                match is_pass_valid {
                    true => Ok(user_db),
                    _ => Err(CustomErrors::NotAuthorized),
                }
            }
            Err(e) => {
                Err(CustomErrors::NotAuthorized)
            }
        }
    }
}
