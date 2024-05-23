#![allow(dead_code)]

use super::custom_error_repo::CustomErrors;
use crate::property::domain_layer::property::Property;
use crate::AppState;
use uuid::Uuid;

pub struct PropertyRepository {
    app_state: AppState,
}

impl PropertyRepository {
    pub async fn new() -> Self {
        PropertyRepository {
            app_state: AppState::new().await,
        }
    }
    pub async fn get_all(&self) -> Result<Vec<Property>, String> {
        let records = sqlx::query_as::<_, Property>("SELECT * FROM property")
            .fetch_all(&self.app_state.database.db)
            .await;

        match records {
            Ok(users) => Ok(users),
            Err(e) => Err(e.to_string()),
        }
    }
    pub async fn get_by_id(&self, id: Uuid) -> Result<Property, String> {
        let record = sqlx::query_as::<_, Property>("SELECT * FROM property WHERE id = $1")
            .bind(id)
            .fetch_one(&self.app_state.database.db)
            .await;

        match record {
            Ok(user) => Ok(user),
            Err(e) => Err(e.to_string()),
        }
    }

    pub async fn save_images(&self, id: Uuid, image_urls: Vec<String>) -> Result<(), String> {
        let table_record_id = sqlx::types::Uuid::from_u128(uuid::Uuid::new_v4().as_u128());
        let record = sqlx::query("INSERT INTO property_photos (property_photos_id, property_id, photo_urls) VALUES ($1, $2, $3)")
            .bind(table_record_id)
            .bind(image_urls)
            .bind(id)
            .execute(&self.app_state.database.db)
            .await;

        match record {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }

    pub async fn create(&self, property: Property) -> Result<Property, String> {
        let property_id = sqlx::types::Uuid::from_u128(uuid::Uuid::new_v4().as_u128());
        let record = sqlx::query("INSERT INTO property (property_id, landlord, secondaryland, branch, lead_staff, onthemarket, status, dateavailable) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)")
            .bind(property_id)
            .bind(&property.landlord)
            .bind(&property.secondaryland)
            .bind(&property.branch)
            .bind(&property.lead_staff)
            .bind(&property.onthemarket)
            .bind(&property.status)
            .bind(&property.dateavailable)
            .execute(&self.app_state.database.db)
            .await;

        match record {
            Ok(_) => Ok(property),
            Err(e) => Err(e.to_string()),
        }
    }

}
