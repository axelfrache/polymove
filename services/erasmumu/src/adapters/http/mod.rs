use crate::application::offer_service::OfferService;
use crate::domain::offer::Offer;
use crate::domain::ports::offer_repository::{OfferError, OfferRepository};
use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize)]
pub struct CreateOfferRequest {
    pub title: String,
    pub link: String,
    pub city: String,
    pub domain: String,
    pub salary: f64,
    pub start_date: String,
    pub end_date: String,
}

#[derive(Deserialize)]
pub struct UpdateOfferRequest {
    pub title: Option<String>,
    pub link: Option<String>,
    pub city: Option<String>,
    pub domain: Option<String>,
    pub salary: Option<f64>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub available: Option<bool>,
}

#[derive(Deserialize)]
pub struct ListParams {
    pub domain: Option<String>,
    pub city: Option<String>,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

impl From<OfferError> for Response {
    fn from(err: OfferError) -> Self {
        let (status, message) = match err {
            OfferError::NotFound => (StatusCode::NOT_FOUND, "Offer not found".to_string()),
            OfferError::InvalidData(msg) => (StatusCode::BAD_REQUEST, msg),
            OfferError::DatabaseError(msg) => {
                tracing::error!("Database error: {}", msg);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                )
            }
        };
        (status, Json(ErrorResponse { error: message })).into_response()
    }
}

type AppResult<T> = Result<T, Response>;

pub fn router<R: OfferRepository + 'static>(service: Arc<OfferService<R>>) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/offer", post(create_offer::<R>).get(list_offers::<R>))
        .route(
            "/offer/{id}",
            get(get_offer::<R>)
                .put(update_offer::<R>)
                .delete(delete_offer::<R>),
        )
        .with_state(service)
}

async fn health() -> impl IntoResponse {
    Json(serde_json::json!({ "status": "ok" }))
}

async fn create_offer<R: OfferRepository>(
    State(service): State<Arc<OfferService<R>>>,
    Json(payload): Json<CreateOfferRequest>,
) -> AppResult<(StatusCode, Json<Offer>)> {
    let offer = service
        .create_offer(
            payload.title,
            payload.link,
            payload.city,
            payload.domain,
            payload.salary,
            payload.start_date,
            payload.end_date,
        )
        .await
        .map_err(Response::from)?;

    Ok((StatusCode::CREATED, Json(offer)))
}

async fn get_offer<R: OfferRepository>(
    State(service): State<Arc<OfferService<R>>>,
    Path(id): Path<String>,
) -> AppResult<Json<Offer>> {
    let offer = service.get_offer(&id).await.map_err(Response::from)?;
    Ok(Json(offer))
}

async fn list_offers<R: OfferRepository>(
    State(service): State<Arc<OfferService<R>>>,
    Query(params): Query<ListParams>,
) -> AppResult<Json<Vec<Offer>>> {
    let offers = if let Some(domain) = params.domain {
        service
            .list_offers_by_domain(&domain)
            .await
            .map_err(Response::from)?
    } else if let Some(city) = params.city {
        service
            .list_offers_by_city(&city)
            .await
            .map_err(Response::from)?
    } else {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "Query parameter 'domain' or 'city' is required".to_string(),
            }),
        )
            .into_response());
    };

    Ok(Json(offers))
}

async fn update_offer<R: OfferRepository>(
    State(service): State<Arc<OfferService<R>>>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateOfferRequest>,
) -> AppResult<Json<Offer>> {
    let offer = service
        .update_offer(
            &id,
            payload.title,
            payload.link,
            payload.city,
            payload.domain,
            payload.salary,
            payload.start_date,
            payload.end_date,
            payload.available,
        )
        .await
        .map_err(Response::from)?;
    Ok(Json(offer))
}

async fn delete_offer<R: OfferRepository>(
    State(service): State<Arc<OfferService<R>>>,
    Path(id): Path<String>,
) -> AppResult<StatusCode> {
    service.delete_offer(&id).await.map_err(Response::from)?;
    Ok(StatusCode::OK)
}
