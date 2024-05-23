#![allow(dead_code)]

use crate::{
    properties::domain_layer::property::{Property, PropertyStatus},
    AppState,
};
use actix_web::web::Json;
use serde_json::json;
use std::sync::Arc;

pub struct PropertyRepository {}

impl PropertyRepository {
    pub fn new() -> Self {
        PropertyRepository {}
    }
    pub async fn get_all(&self, state: Arc<AppState>) -> Result<Vec<Property>, Json<String>> {
        let result = sqlx::query_as::<_, Property>("SELECT * FROM property")
            .fetch_all(&state.db)
            .await;
        match result {
            Ok(landlords) => Ok(landlords),
            Err(e) => Err(Json(e.to_string())),
        }
    }

    pub async fn save(
        &self,
        state: Arc<AppState>,
        property: Property,
    ) -> Result<Property, Json<serde_json::Value>> {
        let status = PropertyStatus::AVAILABLE;
        let result = sqlx::query_as::<_, Property>(
            r#"
            INSERT INTO property (property_id, landlord_id, secondaryland, branch, lead_staff, onthemarket, status, dateavailable)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING *
            "#,
        )
        .bind(&property.property_id)
        .bind(&property.landlord_id)
        .bind(&property.secondaryland)
        .bind(&property.branch)
        .bind(&property.lead_staff)
        .bind(&property.onthemarket)
        .bind(&status)
        .bind(&property.dateavailable)
        .fetch_one(&state.db)
        .await;

        match result {
            Ok(property) => Ok(property),
            Err(e) => match e {
                sqlx::Error::Database(e) => {
                    if let Some(pg_error) = e.constraint() {
                        if pg_error.contains("property_pkey") {
                            Err(Json(
                                json!({ "error": "Duplicate key error", "message": "Property already exists" }),
                            ))
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
}
