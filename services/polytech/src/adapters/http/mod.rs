use crate::adapters::grpc::mi8_client::Mi8GrpcClient;
use crate::application::student_service::StudentService;
use crate::domain::student::Student;
use crate::ports::student_repository::{StudentError, StudentRepository};
use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

struct AppState<R: StudentRepository> {
    service: Arc<StudentService<R>>,
    mi8_client: Mi8GrpcClient,
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

pub async fn router<R: StudentRepository + 'static>(
    service: Arc<StudentService<R>>,
    mi8_client: Mi8GrpcClient,
) -> Router {
    let state = Arc::new(AppState {
        service,
        mi8_client,
    });

    Router::new()
        .route("/health", get(health))
        .route("/student", post(create_student::<R>))
        .route(
            "/student/{id}",
            get(get_student::<R>).delete(delete_student::<R>),
        )
        .route("/mi8/latest", get(get_latest_news::<R>))
        .route("/mi8/latest-in-city", get(get_latest_news_in_city::<R>))
        .with_state(state)
}

async fn health() -> StatusCode {
    StatusCode::OK
}

async fn create_student<R: StudentRepository>(
    State(state): State<Arc<AppState<R>>>,
    Json(payload): Json<CreateStudentRequest>,
) -> AppResult<(StatusCode, Json<Student>)> {
    let student = state.service
        .create_student(payload.firstname, payload.name, payload.domain)
        .await
        .map_err(Response::from)?;

    Ok((StatusCode::CREATED, Json(student)))
}

async fn get_student<R: StudentRepository>(
    State(state): State<Arc<AppState<R>>>,
    Path(id): Path<String>,
) -> AppResult<Json<Student>> {
    let uuid = Uuid::parse_str(&id).map_err(|_| (StatusCode::BAD_REQUEST, "Invalid UUID").into_response())?;
    let student = state.service.get_student(uuid).await.map_err(Response::from)?;
    Ok(Json(student))
}

async fn list_students<R: StudentRepository>(
    State(state): State<Arc<AppState<R>>>,
    Query(params): Query<ListParams>,
) -> AppResult<Json<Vec<Student>>> {
    let students = state.service
        .list_students_by_domain(&params.domain)
        .await
        .map_err(Response::from)?;
    Ok(Json(students))
}

async fn update_student<R: StudentRepository>(
    State(state): State<Arc<AppState<R>>>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateStudentRequest>,
) -> AppResult<Json<Student>> {
    let student = state.service
        .update_student(id, payload.firstname, payload.name, payload.domain)
        .await
        .map_err(Response::from)?;
    Ok(Json(student))
}

async fn delete_student<R: StudentRepository>(
    State(state): State<Arc<AppState<R>>>,
    Path(id): Path<Uuid>,
) -> AppResult<StatusCode> {
    state.service.delete_student(id).await.map_err(Response::from)?;
    Ok(StatusCode::OK)
}


async fn get_latest_news<R: StudentRepository>(
    State(state): State<Arc<AppState<R>>>,
) -> AppResult<Json<Vec<crate::mi8_proto::News>>> {
    let news = state.mi8_client.get_latest_news(10).await.map_err(|e| {
        tracing::error!("MI8 error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, "MI8 Service Unavailable").into_response()
    })?;
    Ok(Json(news))
}

async fn get_latest_news_in_city<R: StudentRepository>(
    State(state): State<Arc<AppState<R>>>,
    Query(params): Query<CityParams>,
) -> AppResult<Json<Vec<crate::mi8_proto::News>>> {
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
