use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::FromRow)]
pub struct Student {
    pub id: Uuid,
    pub firstname: String,
    pub name: String,
    pub domain: String,
}

impl Student {
    pub fn new(firstname: String, name: String, domain: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            firstname,
            name,
            domain,
        }
    }
}
