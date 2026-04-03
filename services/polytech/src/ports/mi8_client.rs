use crate::mi8_proto::{CityScore, News};

pub trait Mi8Client: Send + Sync {
    fn get_latest_news(
        &self,
        limit: i32,
    ) -> impl std::future::Future<Output = Result<Vec<News>, anyhow::Error>> + Send;
    fn get_city_score(
        &self,
        city: String,
    ) -> impl std::future::Future<Output = Result<CityScore, anyhow::Error>> + Send;
    fn get_latest_news_in_city(
        &self,
        city: String,
        limit: i32,
    ) -> impl std::future::Future<Output = Result<Vec<News>, anyhow::Error>> + Send;
}
