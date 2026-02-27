use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErasmumuOffer {
    pub id: String,
    pub title: String,
    pub link: String,
    pub city: String,
    pub domain: String,
    pub salary: f64,
    pub start_date: String,
    pub end_date: String,
}

pub trait ErasmumuClient: Send + Sync {
    fn fetch_offers(
        &self,
        city: Option<String>,
        domain: Option<String>,
    ) -> impl std::future::Future<Output = Result<Vec<ErasmumuOffer>, anyhow::Error>> + Send;
}
