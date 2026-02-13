use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Offer {
    pub id: String,
    pub title: String,
    pub link: String,
    pub city: String,
    pub domain: String,
    pub salary: f64,
    pub start_date: String,
    pub end_date: String,
    pub available: bool,
}
