use crate::domain::model::{CityScore, News};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum NewsError {
    #[error("Database error: {0}")]
    DatabaseError(String),
    #[error("News not found")]
    NotFound,
}

pub trait NewsRepository: Send + Sync {
    fn create_news(
        &self,
        news: &News,
    ) -> impl std::future::Future<Output = Result<(), NewsError>> + Send;

    fn get_latest_news(
        &self,
        limit: i64,
    ) -> impl std::future::Future<Output = Result<Vec<News>, NewsError>> + Send;

    fn get_latest_news_in_city(
        &self,
        city: &str,
        limit: i64,
    ) -> impl std::future::Future<Output = Result<Vec<News>, NewsError>> + Send;

    fn get_city_score(
        &self,
        city: &str,
    ) -> impl std::future::Future<Output = Result<Option<CityScore>, NewsError>> + Send;

    fn update_city_score(
        &self,
        score: &CityScore,
    ) -> impl std::future::Future<Output = Result<(), NewsError>> + Send;

    fn get_top_cities(
        &self,
        limit: i64,
    ) -> impl std::future::Future<Output = Result<Vec<CityScore>, NewsError>> + Send;
}
