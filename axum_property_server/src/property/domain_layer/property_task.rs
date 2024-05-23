use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct PropertyTask {
    pub property_task_id: Uuid,
    pub property_id: Uuid,
    pub description: Option<String>,
    pub task_type: String,
    pub due_date: Option<String>, // Change type to appropriate datetime type if needed
    pub notes: Option<String>,
    pub completed: bool,
    pub when_completed: Option<DateTime<Utc>>, // Change type to appropriate datetime type if needed
    pub task_owner: Option<String>,
    pub sent_email_notification: bool,
    pub task_created: DateTime<Utc>,
}
