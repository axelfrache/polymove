use crate::domain::notification::Notification;
use futures::future::BoxFuture;
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum NotificationError {
    #[error("Notification not found")]
    NotFound,
    #[error("Database error: {0}")]
    DatabaseError(String),
}

pub trait NotificationRepository: Send + Sync {
    fn create<'a>(
        &'a self,
        notification: &'a Notification,
    ) -> BoxFuture<'a, Result<Notification, NotificationError>>;

    fn list_by_student(
        &self,
        student_id: Uuid,
    ) -> BoxFuture<'_, Result<Vec<Notification>, NotificationError>>;

    fn mark_read(&self, id: Uuid) -> BoxFuture<'_, Result<Notification, NotificationError>>;
}
