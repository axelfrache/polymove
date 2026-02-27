pub mod erasmumu_client;
pub mod offers;

use crate::adapters::grpc::mi8_client::Mi8GrpcClient;
use crate::application::student_service::StudentService;
use crate::application::offer_aggregation_service::OfferAggregationService;
use crate::domain::student::Student;
use crate::ports::student_repository::{StudentError, StudentRepository};
use crate::ports::erasmumu_client::ErasmumuClient;
use crate::ports::mi8_client::Mi8Client;
use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub struct AppState<R: StudentRepository, E: ErasmumuClient, M: Mi8Client> {
    pub service: Arc<StudentService<R>>,
    pub mi8_client: Arc<M>,
    pub offer_aggregation_service: Arc<OfferAggregationService<R, E, M>>,
}
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateStudentRequest {
    pub firstname: String,
    pub name: String,
    pub domain: String,
}

#[derive(Deserialize)]
pub struct UpdateStudentRequest {
    pub firstname: Option<String>,
    pub name: Option<String>,
    pub domain: Option<String>,
}

#[derive(Deserialize)]
pub struct ListParams {
    pub domain: String,
}

pub struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

impl From<StudentError> for Response {
    fn from(err: StudentError) -> Self {
        match err {
            StudentError::NotFound => (StatusCode::NOT_FOUND, "Student not found").into_response(),
            StudentError::InvalidData(msg) => (StatusCode::BAD_REQUEST, msg).into_response(),
            StudentError::DatabaseError(msg) => {
                tracing::error!("Database error: {}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response()
            }
        }
    }
}

type AppResult<T> = Result<T, Response>;

pub async fn router<R, E, M>(
    service: Arc<StudentService<R>>,
    mi8_client: Arc<M>,
    offer_aggregation_service: Arc<OfferAggregationService<R, E, M>>,
) -> Router
where
    R: StudentRepository + Send + Sync + 'static,
    E: ErasmumuClient + Send + Sync + 'static,
    M: Mi8Client + Send + Sync + 'static,
{
    let state = Arc::new(AppState {
        service,
        mi8_client,
        offer_aggregation_service,
    });

    Router::<Arc<AppState<R, E, M>>>::new()
        .route("/health", get(health))
        .route("/student", post(create_student::<R, E, M>))
        .route(
            "/student/{id}",
            get(get_student::<R, E, M>).delete(delete_student::<R, E, M>),
        )
        .route("/mi8/latest", get(get_latest_news::<R, E, M>))
        .route("/mi8/latest-in-city", get(get_latest_news_in_city::<R, E, M>))
        .route("/offers", get(offers::get_offers::<R, E, M>))
        .route("/students/{id}/recommended-offers", get(offers::get_recommended_offers::<R, E, M>))
        .with_state(state)
}

async fn health() -> StatusCode {
    StatusCode::OK
}

async fn create_student<R, E, M>(
    State(state): State<Arc<AppState<R, E, M>>>,
    Json(payload): Json<CreateStudentRequest>,
) -> AppResult<(StatusCode, Json<Student>)> 
where
    R: StudentRepository + Send + Sync + 'static,
    E: ErasmumuClient + Send + Sync + 'static,
    M: Mi8Client + Send + Sync + 'static,
{
    let student = state.service
        .create_student(payload.firstname, payload.name, payload.domain)
        .await
        .map_err(Response::from)?;

    Ok((StatusCode::CREATED, Json(student)))
}

async fn get_student<R, E, M>(
    State(state): State<Arc<AppState<R, E, M>>>,
    Path(id): Path<String>,
) -> AppResult<Json<Student>> 
where
    R: StudentRepository + Send + Sync + 'static,
    E: ErasmumuClient + Send + Sync + 'static,
    M: Mi8Client + Send + Sync + 'static,
{
    let uuid = Uuid::parse_str(&id).map_err(|_| (StatusCode::BAD_REQUEST, "Invalid UUID").into_response())?;
    let student = state.service.get_student(uuid).await.map_err(Response::from)?;
    Ok(Json(student))
}

async fn list_students<R, E, M>(
    State(state): State<Arc<AppState<R, E, M>>>,
    Query(params): Query<ListParams>,
) -> AppResult<Json<Vec<Student>>> 
where
    R: StudentRepository + Send + Sync + 'static,
    E: ErasmumuClient + Send + Sync + 'static,
    M: Mi8Client + Send + Sync + 'static,
{
    let students = state.service
        .list_students_by_domain(&params.domain)
        .await
        .map_err(Response::from)?;
    Ok(Json(students))
}

async fn update_student<R, E, M>(
    State(state): State<Arc<AppState<R, E, M>>>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateStudentRequest>,
) -> AppResult<Json<Student>> 
where
    R: StudentRepository + Send + Sync + 'static,
    E: ErasmumuClient + Send + Sync + 'static,
    M: Mi8Client + Send + Sync + 'static,
{
    let student = state.service
        .update_student(id, payload.firstname, payload.name, payload.domain)
        .await
        .map_err(Response::from)?;
    Ok(Json(student))
}

async fn delete_student<R, E, M>(
    State(state): State<Arc<AppState<R, E, M>>>,
    Path(id): Path<Uuid>,
) -> AppResult<StatusCode> 
where
    R: StudentRepository + Send + Sync + 'static,
    E: ErasmumuClient + Send + Sync + 'static,
    M: Mi8Client + Send + Sync + 'static,
{
    state.service.delete_student(id).await.map_err(Response::from)?;
    Ok(StatusCode::OK)
}


async fn get_latest_news<R, E, M>(
    State(state): State<Arc<AppState<R, E, M>>>,
) -> AppResult<Json<Vec<crate::mi8_proto::News>>> 
where
    R: StudentRepository + Send + Sync + 'static,
    E: ErasmumuClient + Send + Sync + 'static,
    M: Mi8Client + Send + Sync + 'static,
{
    let news = state.mi8_client.get_latest_news(10).await.map_err(|e| {
        tracing::error!("MI8 error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, "MI8 Service Unavailable").into_response()
    })?;
    Ok(Json(news))
}

async fn get_latest_news_in_city<R, E, M>(
    State(state): State<Arc<AppState<R, E, M>>>,
    Query(params): Query<CityParams>,
) -> AppResult<Json<Vec<crate::mi8_proto::News>>> 
where
    R: StudentRepository + Send + Sync + 'static,
    E: ErasmumuClient + Send + Sync + 'static,
    M: Mi8Client + Send + Sync + 'static,
{
    let news = state.mi8_client.get_latest_news_in_city(params.city, 10).await.map_err(|e| {
        tracing::error!("MI8 error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, "MI8 Service Unavailable").into_response()
    })?;
    Ok(Json(news))
}

#[derive(Deserialize)]
pub struct CityParams {
    pub city: String,
}
