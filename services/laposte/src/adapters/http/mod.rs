use crate::application::subscriber_service::SubscriberService;
use crate::domain::subscriber::Subscriber;
use crate::ports::subscriber_repository::{SubscriberError, SubscriberRepository};
use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

impl From<SubscriberError> for Response {
    fn from(err: SubscriberError) -> Self {
        let (status, message) = match err {
            SubscriberError::NotFound => {
                (StatusCode::NOT_FOUND, "Subscriber not found".to_string())
            }
            SubscriberError::DatabaseError(msg) => {
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

pub fn router<R: SubscriberRepository + 'static>(
    service: Arc<SubscriberService<R>>,
) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .route("/health", get(health))
        .route(
            "/subscribers/{studentId}",
            get(get_subscriber::<R>)
                .put(update_subscriber::<R>)
                .delete(delete_subscriber::<R>),
        )
        .layer(cors)
        .with_state(service)
}

async fn health() -> impl IntoResponse {
    Json(serde_json::json!({ "status": "ok" }))
}

async fn get_subscriber<R: SubscriberRepository>(
    State(service): State<Arc<SubscriberService<R>>>,
    Path(student_id): Path<String>,
) -> AppResult<Json<Subscriber>> {
    let subscriber = service
        .get_subscriber(&student_id)
        .await
        .map_err(Response::from)?;
    Ok(Json(subscriber))
}

#[derive(Deserialize)]
struct UpdateSubscriberRequest {
    pub channel: Option<String>,
    pub contact: Option<String>,
    pub enabled: Option<bool>,
}

async fn update_subscriber<R: SubscriberRepository>(
    State(service): State<Arc<SubscriberService<R>>>,
    Path(student_id): Path<String>,
    Json(payload): Json<UpdateSubscriberRequest>,
) -> AppResult<Json<Subscriber>> {
    let subscriber = service
        .update_subscriber(&student_id, payload.channel, payload.contact, payload.enabled)
        .await
        .map_err(Response::from)?;
    Ok(Json(subscriber))
}

async fn delete_subscriber<R: SubscriberRepository>(
    State(service): State<Arc<SubscriberService<R>>>,
    Path(student_id): Path<String>,
) -> AppResult<StatusCode> {
    service
        .unsubscribe(&student_id)
        .await
        .map_err(Response::from)?;
    Ok(StatusCode::OK)
}
