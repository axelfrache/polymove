use crate::domain::model::{CityScore, News};
use crate::domain::ports::news_repository::{NewsError, NewsRepository};
use redis::aio::ConnectionManager; 
use redis::FromRedisValue; // Import essential trait

pub struct RedisNewsRepository {
    con_manager: ConnectionManager,
}

impl RedisNewsRepository {
    pub fn new(con_manager: ConnectionManager) -> Self {
        Self { con_manager }
    }
}

impl NewsRepository for RedisNewsRepository {
    async fn create_news(&self, news: &News) -> Result<(), NewsError> {
        let mut con = self.con_manager.clone();
        let serialized_news = serde_json::to_string(news)
            .map_err(|e| NewsError::DatabaseError(e.to_string()))?;
        
        // 1. Store news payload
        // HSET key field value
        redis::cmd("HSET")
            .arg("news")
            .arg(&news.id)
            .arg(&serialized_news)
            .query_async::<()>(&mut con)
            .await
            .map_err(|e| NewsError::DatabaseError(e.to_string()))?;
        
        // 2. Add to global timeline (ZSET, score = timestamp)
        let timestamp = chrono::Utc::now().timestamp_millis();
        redis::cmd("ZADD")
            .arg("timeline:global")
            .arg(timestamp)
            .arg(&news.id)
            .query_async::<()>(&mut con)
            .await
            .map_err(|e| NewsError::DatabaseError(e.to_string()))?;
        
        // 3. Add to city timeline
        let city_key = format!("timeline:city:{}", news.city);
        redis::cmd("ZADD")
            .arg(&city_key)
            .arg(timestamp)
            .arg(&news.id)
            .query_async::<()>(&mut con)
            .await
            .map_err(|e| NewsError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn get_latest_news(&self, limit: i64) -> Result<Vec<News>, NewsError> {
        let mut con = self.con_manager.clone();
        
        let ids: Vec<String> = redis::cmd("ZREVRANGE")
            .arg("timeline:global")
            .arg(0)
            .arg(limit - 1)
            .query_async::<Vec<String>>(&mut con)
            .await
            .map_err(|e| NewsError::DatabaseError(e.to_string()))?;

        if ids.is_empty() {
            return Ok(Vec::new());
        }

        // Fetch values as generic Redis Values
        let values: Vec<redis::Value> = redis::cmd("HMGET")
            .arg("news")
            .arg(&ids)
            .query_async::<Vec<redis::Value>>(&mut con)
            .await
            .map_err(|e| NewsError::DatabaseError(e.to_string()))?;

        let mut news_list = Vec::new();
        for val in values {
            // Use String::from_redis_value for robustness
            if let Ok(s) = String::from_redis_value(&val) {
                 if let Ok(n) = serde_json::from_str::<News>(&s) {
                    news_list.push(n);
                }
            }
        }

        Ok(news_list)
    }

    async fn get_latest_news_in_city(
        &self,
        city: &str,
        limit: i64,
    ) -> Result<Vec<News>, NewsError> {
        let mut con = self.con_manager.clone();
        let city_key = format!("timeline:city:{}", city);

        let ids: Vec<String> = redis::cmd("ZREVRANGE")
            .arg(&city_key)
            .arg(0)
            .arg(limit - 1)
            .query_async::<Vec<String>>(&mut con)
            .await
            .map_err(|e| NewsError::DatabaseError(e.to_string()))?;

        if ids.is_empty() {
            return Ok(Vec::new());
        }

        let values: Vec<redis::Value> = redis::cmd("HMGET")
            .arg("news")
            .arg(&ids)
            .query_async::<Vec<redis::Value>>(&mut con)
            .await
            .map_err(|e| NewsError::DatabaseError(e.to_string()))?;

        let mut news_list = Vec::new();
        for val in values {
             if let Ok(s) = String::from_redis_value(&val) {
                if let Ok(n) = serde_json::from_str::<News>(&s) {
                    news_list.push(n);
                }
            }
        }

        Ok(news_list)
    }

    async fn get_city_score(&self, city: &str) -> Result<Option<CityScore>, NewsError> {
        let mut con = self.con_manager.clone();
        let key = format!("score:{}", city);
        
        let exists: bool = redis::cmd("EXISTS")
            .arg(&key)
            .query_async::<bool>(&mut con)
            .await
            .unwrap_or(false);
            
        if !exists {
            return Ok(None);
        }

        let payload: String = redis::cmd("GET")
            .arg(&key)
            .query_async::<String>(&mut con)
            .await
            .map_err(|e| NewsError::DatabaseError(e.to_string()))?;
            
        let score = serde_json::from_str(&payload).map_err(|e| NewsError::DatabaseError(e.to_string()))?;
        Ok(Some(score))
    }

    async fn update_city_score(&self, score: &CityScore) -> Result<(), NewsError> {
        let mut con = self.con_manager.clone();
        let key = format!("score:{}", score.city);
        let serialized = serde_json::to_string(score).map_err(|e| NewsError::DatabaseError(e.to_string()))?;

        redis::cmd("SET")
            .arg(&key)
            .arg(&serialized)
            .query_async::<()>(&mut con)
            .await
            .map_err(|e| NewsError::DatabaseError(e.to_string()))?;
        
        redis::cmd("ZADD")
            .arg("leaderboard:global")
            .arg(score.total_score())
            .arg(&score.city)
            .query_async::<()>(&mut con)
            .await
            .map_err(|e| NewsError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn get_top_cities(&self, limit: i64) -> Result<Vec<CityScore>, NewsError> {
        let mut con = self.con_manager.clone();
        
        let cities: Vec<String> = redis::cmd("ZRANGE")
            .arg("leaderboard:global")
            .arg(0)
            .arg(limit - 1)
            .query_async::<Vec<String>>(&mut con)
            .await
            .map_err(|e| NewsError::DatabaseError(e.to_string()))?;

        let mut scores = Vec::new();
        for city in cities {
            if let Ok(Some(score)) = self.get_city_score(&city).await {
                scores.push(score);
            }
        }

        Ok(scores)
    }
}
