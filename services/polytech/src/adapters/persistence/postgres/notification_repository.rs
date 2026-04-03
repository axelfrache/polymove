use crate::domain::notification::Notification;
use crate::ports::notification_repository::{NotificationError, NotificationRepository};
use futures::future::BoxFuture;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone)]
pub struct PostgresNotificationRepository {
    pool: PgPool,
}

impl PostgresNotificationRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl NotificationRepository for PostgresNotificationRepository {
    fn create<'a>(
        &'a self,
        notification: &'a Notification,
    ) -> BoxFuture<'a, Result<Notification, NotificationError>> {
        Box::pin(async move {
            sqlx::query_as::<_, Notification>(
                r#"
                INSERT INTO notifications (id, student_id, notification_type, offer_id, message, read)
                VALUES ($1, $2, $3, $4, $5, $6)
                RETURNING id, student_id, notification_type, offer_id, message, read
                "#,
            )
            .bind(notification.id)
            .bind(notification.student_id)
            .bind(&notification.notification_type)
            .bind(&notification.offer_id)
            .bind(&notification.message)
            .bind(notification.read)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| NotificationError::DatabaseError(e.to_string()))
        })
    }

    fn list_by_student(
        &self,
        student_id: Uuid,
    ) -> BoxFuture<'_, Result<Vec<Notification>, NotificationError>> {
        Box::pin(async move {
            sqlx::query_as::<_, Notification>(
                r#"
                SELECT id, student_id, notification_type, offer_id, message, read
                FROM notifications
                WHERE student_id = $1
                ORDER BY id DESC
                "#,
            )
            .bind(student_id)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| NotificationError::DatabaseError(e.to_string()))
        })
    }

    fn mark_read(&self, id: Uuid) -> BoxFuture<'_, Result<Notification, NotificationError>> {
        Box::pin(async move {
            sqlx::query_as::<_, Notification>(
                r#"
                UPDATE notifications
                SET read = true
                WHERE id = $1
                RETURNING id, student_id, notification_type, offer_id, message, read
                "#,
            )
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| NotificationError::DatabaseError(e.to_string()))?
            .ok_or(NotificationError::NotFound)
        })
    }
}
