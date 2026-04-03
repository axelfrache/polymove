use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Notification {
    pub id: Uuid,
    pub student_id: Uuid,
    pub notification_type: String,
    pub offer_id: String,
    pub message: String,
    pub read: bool,
}

impl Notification {
    pub fn new_offer(student_id: Uuid, offer_id: String, message: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            student_id,
            notification_type: "new_offer".to_string(),
            offer_id,
            message,
            read: false,
        }
    }
}
