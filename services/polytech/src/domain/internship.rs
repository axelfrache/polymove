use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Internship {
    pub id: Uuid,
    pub student_id: Uuid,
    pub offer_id: String,
    pub approved: bool,
    pub message: String,
}

impl Internship {
    pub fn new(student_id: Uuid, offer_id: String, approved: bool, message: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            student_id,
            offer_id,
            approved,
            message,
        }
    }
}
