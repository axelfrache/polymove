use crate::application::news_service::NewsService;
use crate::domain::model::{CityScore, News};
use crate::domain::ports::news_repository::NewsRepository;
use crate::mi8_proto::mi8_service_server::Mi8Service;
use crate::mi8_proto::{
    CityScore as ProtoCityScore, CreateNewsRequest, CreateNewsResponse, GetCityScoreRequest,
    GetCityScoreResponse, GetLatestNewsInCityRequest, GetLatestNewsInCityResponse,
    GetLatestNewsRequest, GetLatestNewsResponse, GetTopCitiesRequest, GetTopCitiesResponse,
    News as ProtoNews,
};
use std::sync::Arc;
use tonic::{Request, Response, Status};

pub struct Mi8ServiceImpl<R: NewsRepository> {
    service: Arc<NewsService<R>>,
}

impl<R: NewsRepository> Mi8ServiceImpl<R> {
    pub fn new(service: Arc<NewsService<R>>) -> Self {
        Self { service }
    }
}

impl From<News> for ProtoNews {
    fn from(n: News) -> Self {
        ProtoNews {
            id: n.id,
            name: n.name,
            source: n.source,
            date: n.date,
            tags: n.tags,
            city: n.city,
            country: n.country,
        }
    }
}

impl From<ProtoNews> for News {
    fn from(n: ProtoNews) -> Self {
        News {
            id: n.id,
            name: n.name,
            source: n.source,
            date: n.date,
            tags: n.tags,
            city: n.city,
            country: n.country,
        }
    }
}

impl From<CityScore> for ProtoCityScore {
    fn from(c: CityScore) -> Self {
        ProtoCityScore {
            city: c.city,
            country: c.country,
            quality_of_life: c.quality_of_life,
            safety: c.safety,
            economy: c.economy,
            culture: c.culture,
            last_updated: c.last_updated,
        }
    }
}

#[tonic::async_trait]
impl<R: NewsRepository + 'static> Mi8Service for Mi8ServiceImpl<R> {
    async fn get_latest_news(
        &self,
        request: Request<GetLatestNewsRequest>,
    ) -> Result<Response<GetLatestNewsResponse>, Status> {
        let limit = request.into_inner().limit as i64;
        let news = self
            .service
            .get_latest_news(limit)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(GetLatestNewsResponse {
            news: news.into_iter().map(Into::into).collect(),
        }))
    }

    async fn get_latest_news_in_city(
        &self,
        request: Request<GetLatestNewsInCityRequest>,
    ) -> Result<Response<GetLatestNewsInCityResponse>, Status> {
        let req = request.into_inner();
        let news = self
            .service
            .get_latest_news_in_city(&req.city, req.limit as i64)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(GetLatestNewsInCityResponse {
            news: news.into_iter().map(Into::into).collect(),
        }))
    }

    async fn create_news(
        &self,
        request: Request<CreateNewsRequest>,
    ) -> Result<Response<CreateNewsResponse>, Status> {
        let req = request.into_inner();
        let proto_news = req.news.ok_or(Status::invalid_argument("Missing news"))?;
        let news: News = proto_news.into();

        self.service
            .create_news(news.clone())
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(CreateNewsResponse {
            success: true,
            message: "News created".to_string(),
            news_id: news.id,
        }))
    }

    async fn get_city_score(
        &self,
        request: Request<GetCityScoreRequest>,
    ) -> Result<Response<GetCityScoreResponse>, Status> {
        let city = request.into_inner().city;
        let score = self
            .service
            .get_city_score(&city)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(GetCityScoreResponse {
            score: Some(score.into()),
        }))
    }

    async fn get_top_cities(
        &self,
        request: Request<GetTopCitiesRequest>,
    ) -> Result<Response<GetTopCitiesResponse>, Status> {
        let limit = request.into_inner().limit as i64;
        let scores = self
            .service
            .get_top_cities(limit)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(GetTopCitiesResponse {
            scores: scores.into_iter().map(Into::into).collect(),
        }))
    }
}
