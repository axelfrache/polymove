use crate::application::student_service::StudentService;
use crate::domain::student::Student;
use crate::ports::student_repository::{StudentError, StudentRepository};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use std::sync::Arc;
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

pub fn router<R: StudentRepository + 'static>(service: Arc<StudentService<R>>) -> Router {
    Router::new()
        .route(
            "/student",
            post(create_student::<R>).get(list_students::<R>),
        )
        .route(
            "/student/{id}",
            get(get_student::<R>)
                .put(update_student::<R>)
                .delete(delete_student::<R>),
        )
        .with_state(service)
}

async fn create_student<R: StudentRepository>(
    State(service): State<Arc<StudentService<R>>>,
    Json(payload): Json<CreateStudentRequest>,
) -> AppResult<(StatusCode, Json<Student>)> {
    let student = service
        .create_student(payload.firstname, payload.name, payload.domain)
        .await
        .map_err(Response::from)?;

    Ok((StatusCode::CREATED, Json(student)))
}

async fn get_student<R: StudentRepository>(
    State(service): State<Arc<StudentService<R>>>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<Student>> {
    let student = service.get_student(id).await.map_err(Response::from)?;
    Ok(Json(student))
}

async fn list_students<R: StudentRepository>(
    State(service): State<Arc<StudentService<R>>>,
    Query(params): Query<ListParams>,
) -> AppResult<Json<Vec<Student>>> {
    let students = service
        .list_students_by_domain(&params.domain)
        .await
        .map_err(Response::from)?;
    Ok(Json(students))
}

async fn update_student<R: StudentRepository>(
    State(service): State<Arc<StudentService<R>>>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateStudentRequest>,
) -> AppResult<Json<Student>> {
    let student = service
        .update_student(id, payload.firstname, payload.name, payload.domain)
        .await
        .map_err(Response::from)?;
    Ok(Json(student))
}

async fn delete_student<R: StudentRepository>(
    State(service): State<Arc<StudentService<R>>>,
    Path(id): Path<Uuid>,
) -> AppResult<StatusCode> {
    service.delete_student(id).await.map_err(Response::from)?;
    Ok(StatusCode::OK)
}
