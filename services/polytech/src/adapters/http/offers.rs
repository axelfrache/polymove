use crate::adapters::http::AppState;
use crate::application::offer_aggregation_service::{EnrichedOffer, OfferAggregationService};
use crate::domain::student::Student;
use crate::ports::erasmumu_client::ErasmumuClient;
use crate::ports::mi8_client::Mi8Client;
use crate::ports::student_repository::StudentRepository;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct OffersQuery {
    #[serde(default = "default_limit")]
    pub limit: usize,
    pub city: Option<String>,
    pub domain: Option<String>,
}

fn default_limit() -> usize {
    10
}

#[derive(Deserialize)]
pub struct RecommendedOffersQuery {
    #[serde(default = "default_recommended_limit")]
    pub limit: usize,
    pub sort_by: Option<String>,
}

fn default_recommended_limit() -> usize {
    5
}

#[derive(Serialize)]
pub struct OffersResponse {
    pub offers: Vec<EnrichedOffer>,
}

#[derive(Serialize)]
pub struct RecommendedOffersResponse {
    pub student: Student,
    pub offers: Vec<EnrichedOffer>,
}

pub async fn get_offers<R, E, M>(
    State(state): State<Arc<AppState<R, E, M>>>,
    Query(params): Query<OffersQuery>,
) -> Result<Json<OffersResponse>, Response>
where
    R: StudentRepository + Send + Sync + 'static,
    E: ErasmumuClient + Send + Sync + 'static,
    M: Mi8Client + Send + Sync + 'static,
{
    match state
        .offer_aggregation_service
        .get_enriched_offers(params.city, params.domain, params.limit)
        .await
    {
        Ok(offers) => Ok(Json(OffersResponse { offers })),
        Err(e) => {
            tracing::error!("Failed to fetch aggregated offers: {}", e);
            Err((
                StatusCode::SERVICE_UNAVAILABLE,
                "Failed to aggregate offers. Dependent service might be unavailable.",
            )
                .into_response())
        }
    }
}

pub async fn get_recommended_offers<R, E, M>(
    State(state): State<Arc<AppState<R, E, M>>>,
    Path(id): Path<String>,
    Query(params): Query<RecommendedOffersQuery>,
) -> Result<Json<RecommendedOffersResponse>, Response>
where
    R: StudentRepository + Send + Sync + 'static,
    E: ErasmumuClient + Send + Sync + 'static,
    M: Mi8Client + Send + Sync + 'static,
{
    let uuid = match Uuid::parse_str(&id) {
        Ok(u) => u,
        Err(_) => return Err((StatusCode::BAD_REQUEST, "Invalid student UUID").into_response()),
    };

    match state
        .offer_aggregation_service
        .get_recommended_offers(uuid, params.limit, params.sort_by)
        .await
    {
        Ok((student, offers)) => Ok(Json(RecommendedOffersResponse { student, offers })),
        Err(e) => {
            let err_msg = format!("{}", e);
            tracing::error!("Failed to fetch recommended offers: {}", err_msg);
            if err_msg.contains("Student not found") {
                Err((StatusCode::NOT_FOUND, "Student not found").into_response())
            } else {
                Err((
                    StatusCode::SERVICE_UNAVAILABLE,
                    "Failed to fetch recommended offers.",
                )
                    .into_response())
            }
        }
    }
}
