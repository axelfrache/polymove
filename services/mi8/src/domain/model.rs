use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct News {
    pub id: String,
    pub name: String,
    pub source: String,
    pub date: String,
    pub tags: Vec<String>,
    pub city: String,
    pub country: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CityScore {
    pub city: String,
    pub country: String,
    pub quality_of_life: i32,
    pub safety: i32,
    pub economy: i32,
    pub culture: i32,
    pub last_updated: String,
}

impl CityScore {
    pub fn new(city: String, country: String) -> Self {
        Self {
            city,
            country,
            quality_of_life: 1000,
            safety: 1000,
            economy: 1000,
            culture: 1000,
            last_updated: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn total_score(&self) -> i32 {
        self.quality_of_life + self.safety + self.economy + self.culture
    }
}
