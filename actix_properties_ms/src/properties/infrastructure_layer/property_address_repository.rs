#![allow(dead_code)]
use std::sync::Arc;

use crate::{properties::domain_layer::property_address::PropertyAddress, AppState};
use actix_web::web::Json;
use uuid::Uuid;

pub struct PropertyAddressRepository {}

impl PropertyAddressRepository {
    pub fn new() -> Self {
        PropertyAddressRepository {}
    }
    pub async fn get_all(
        &self,
        state: Arc<AppState>,
    ) -> Result<Vec<PropertyAddress>, Json<String>> {
        let result = sqlx::query_as::<_, PropertyAddress>("SELECT * FROM property_address")
            .fetch_all(&state.db)
            .await;
        match result {
            Ok(property_address) => Ok(property_address),
            Err(e) => Err(Json(e.to_string())),
        }
    }

    pub async fn save(
        &self,
        state: Arc<AppState>,
        property_address: PropertyAddress,
    ) -> Result<PropertyAddress, Json<String>> {
        let result = sqlx::query_as::<_, PropertyAddress>(
            r#"
            INSERT INTO property_address (address_id, property_id, display_add, add_line1, add_line2, towncity, county, country, searchable_areas)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING *
            "#,
        )
        .bind(&property_address.address_id)
        .bind(&property_address.property_id)
        .bind(&property_address.display_add)
        .bind(&property_address.add_line1)
        .bind(&property_address.add_line2)
        .bind(&property_address.towncity)
        .bind(&property_address.county)
        .bind(&property_address.country)
        .bind(&property_address.searchable_areas)
        .fetch_one(&state.db)
        .await;

        match result {
            Ok(property_address) => Ok(property_address),
            Err(e) => Err(Json(e.to_string())),
        }
    }

    pub async fn get_by_id(
        &self,
        state: Arc<AppState>,
        property_id: Uuid,
    ) -> Result<Vec<PropertyAddress>, Json<String>> {
        let result = sqlx::query_as::<_, PropertyAddress>(
            "SELECT * FROM property_address WHERE property_id = $1",
        )
        .bind(&property_id)
        .fetch_all(&state.db)
        .await;

        match result {
            Ok(property_address) => Ok(property_address),
            Err(e) => Err(Json(e.to_string())),
        }
    }

    pub async fn update(
        &self,
        state: Arc<AppState>,
        property_address: PropertyAddress,
    ) -> Result<PropertyAddress, Json<String>> {
        let result = sqlx::query_as::<_, PropertyAddress>(
            r#"
            UPDATE property_address
            SET display_add = $1, add_line1 = $2, add_line2 = $3, towncity = $4, county = $5, country = $6, searchable_areas = $7
            WHERE property_id = $8
            RETURNING *
            "#,
        )
        .bind(&property_address.display_add)
        .bind(&property_address.add_line1)
        .bind(&property_address.add_line2)
        .bind(&property_address.towncity)
        .bind(&property_address.county)
        .bind(&property_address.country)
        .bind(&property_address.searchable_areas)
        .bind(&property_address.property_id)
        .fetch_one(&state.db)
        .await;

        match result {
            Ok(property_address) => Ok(property_address),
            Err(e) => Err(Json(e.to_string())),
        }
    }
}
