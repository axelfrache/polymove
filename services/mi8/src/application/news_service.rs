use crate::domain::model::{CityScore, News};
use crate::domain::ports::news_repository::{NewsError, NewsRepository};

pub struct NewsService<R: NewsRepository> {
    repository: R,
}

impl<R: NewsRepository> NewsService<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn create_news(&self, mut news: News) -> Result<(), NewsError> {
        // Assign ID if missing (though usually done by caller or client, let's ensure it)
        if news.id.is_empty() {
            news.id = uuid::Uuid::new_v4().to_string();
        }

        // 1. Get current city score or init default
        let mut score = self
            .repository
            .get_city_score(&news.city)
            .await?
            .unwrap_or_else(|| CityScore::new(news.city.clone(), news.country.clone()));

        // 2. Apply scoring rules based on tags
        for tag in &news.tags {
            match tag.as_str() {
                "innovation" => {
                    score.quality_of_life += 5;
                    score.economy += 10;
                    score.culture += 2;
                }
                "culture" => {
                    score.quality_of_life += 2;
                    score.culture += 10;
                }
                "healthcare" => {
                    score.quality_of_life += 8;
                    score.safety += 2;
                }
                "entertainment" => {
                    score.quality_of_life += 3;
                    score.economy += 5;
                    score.culture += 5;
                }
                "crisis" => {
                    score.quality_of_life -= 10;
                    score.safety -= 5;
                    score.economy -= 10;
                }
                "crime" => {
                    score.quality_of_life -= 5;
                    score.safety -= 10;
                    score.economy -= 2;
                }
                "disaster" => {
                    score.quality_of_life -= 15;
                    score.safety -= 15;
                    score.economy -= 20;
                    score.culture -= 5;
                }
                _ => {}
            }
        }

        // 3. Ensure scores don't drop below 0
        score.quality_of_life = score.quality_of_life.max(0);
        score.safety = score.safety.max(0);
        score.economy = score.economy.max(0);
        score.culture = score.culture.max(0);
        score.last_updated = chrono::Utc::now().to_rfc3339();

        // 4. Update repository (news + score)
        // Ideally transactional, but here sequential
        self.repository.create_news(&news).await?;
        self.repository.update_city_score(&score).await?;

        Ok(())
    }

    pub async fn get_latest_news(&self, limit: i64) -> Result<Vec<News>, NewsError> {
        self.repository.get_latest_news(limit).await
    }

    pub async fn get_latest_news_in_city(
        &self,
        city: &str,
        limit: i64,
    ) -> Result<Vec<News>, NewsError> {
        self.repository.get_latest_news_in_city(city, limit).await
    }

    pub async fn get_city_score(&self, city: &str) -> Result<CityScore, NewsError> {
        let score = self
            .repository
            .get_city_score(city)
            .await?
            .ok_or(NewsError::NotFound)?;
        Ok(score)
    }

    pub async fn get_top_cities(&self, limit: i64) -> Result<Vec<CityScore>, NewsError> {
        self.repository.get_top_cities(limit).await
    }
}
