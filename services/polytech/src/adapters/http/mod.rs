pub mod erasmumu_client;
pub mod offers;

use crate::adapters::grpc::mi8_client::Mi8GrpcClient;
use crate::application::student_service::StudentService;
use crate::application::offer_aggregation_service::OfferAggregationService;
use crate::domain::internship::Internship;
use crate::domain::student::Student;
use crate::ports::student_repository::{StudentError, StudentRepository};
use crate::ports::erasmumu_client::ErasmumuClient;
use crate::ports::internship_repository::InternshipRepository;
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
    pub erasmumu_client: Arc<E>,
    pub mi8_client: Arc<M>,
    pub offer_aggregation_service: Arc<OfferAggregationService<R, E, M>>,
    pub internship_repository: Arc<dyn InternshipRepository>,
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
    erasmumu_client: Arc<E>,
    mi8_client: Arc<M>,
    offer_aggregation_service: Arc<OfferAggregationService<R, E, M>>,
    internship_repository: Arc<dyn InternshipRepository>,
) -> Router
where
    R: StudentRepository + Send + Sync + 'static,
    E: ErasmumuClient + Send + Sync + 'static,
    M: Mi8Client + Send + Sync + 'static,
{
    let state = Arc::new(AppState {
        service,
        erasmumu_client,
        mi8_client,
        offer_aggregation_service,
        internship_repository,
    });

    let cors = tower_http::cors::CorsLayer::permissive();

    Router::<Arc<AppState<R, E, M>>>::new()
        .route("/health", get(health))
        .route("/student", post(create_student::<R, E, M>).get(list_students::<R, E, M>))
        .route(
            "/student/{id}",
            get(get_student::<R, E, M>).delete(delete_student::<R, E, M>),
        )
        .route("/mi8/latest", get(get_latest_news::<R, E, M>))
        .route("/mi8/latest-in-city", get(get_latest_news_in_city::<R, E, M>))
        .route("/offers", get(offers::get_offers::<R, E, M>))
        .route("/students/{id}/recommended-offers", get(offers::get_recommended_offers::<R, E, M>))
        .route("/internship", post(apply_internship::<R, E, M>))
        .route("/internship/{id}", get(get_internship::<R, E, M>))
        .layer(cors)
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

#[derive(Deserialize)]
pub struct InternshipRequest {
    #[serde(rename = "studentId")]
    pub student_id: String,
    #[serde(rename = "offerId")]
    pub offer_id: String,
}

#[derive(Serialize)]
pub struct InternshipResponse {
    pub approved: bool,
    pub message: String,
}

async fn apply_internship<R, E, M>(
    State(state): State<Arc<AppState<R, E, M>>>,
    Json(payload): Json<InternshipRequest>,
) -> AppResult<Json<InternshipResponse>>
where
    R: StudentRepository + Send + Sync + 'static,
    E: ErasmumuClient + Send + Sync + 'static,
    M: Mi8Client + Send + Sync + 'static,
{
    let uuid = Uuid::parse_str(&payload.student_id)
        .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid student UUID").into_response())?;

    state.service.get_student(uuid).await.map_err(Response::from)?;

    let approved = state
        .erasmumu_client
        .register_internship(&payload.offer_id)
        .await
        .map_err(|e| {
            tracing::error!("Erasmumu internship registration error: {}", e);
            (StatusCode::SERVICE_UNAVAILABLE, "Erasmumu service unavailable").into_response()
        })?;

    let message = if approved {
        "Application successfully approved!".to_string()
    } else {
        "Application rejected: offer is no longer available.".to_string()
    };

    let internship = Internship::new(uuid, payload.offer_id, approved, message.clone());
    state
        .internship_repository
        .save(&internship)
        .await
        .map_err(|e| {
            tracing::error!("Failed to persist internship: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response()
        })?;

    Ok(Json(InternshipResponse { approved, message }))
}

async fn get_internship<R, E, M>(
    State(state): State<Arc<AppState<R, E, M>>>,
    Path(id): Path<String>,
) -> AppResult<Json<Internship>>
where
    R: StudentRepository + Send + Sync + 'static,
    E: ErasmumuClient + Send + Sync + 'static,
    M: Mi8Client + Send + Sync + 'static,
{
    let uuid = Uuid::parse_str(&id)
        .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid internship UUID").into_response())?;

    state
        .internship_repository
        .get(uuid)
        .await
        .map(Json)
        .map_err(|e| match e {
            crate::ports::internship_repository::InternshipError::NotFound => {
                (StatusCode::NOT_FOUND, "Internship not found").into_response()
            }
            crate::ports::internship_repository::InternshipError::DatabaseError(msg) => {
                tracing::error!("Database error: {}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response()
            }
        })
}
