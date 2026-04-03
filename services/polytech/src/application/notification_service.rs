use crate::domain::notification::Notification;
use crate::ports::notification_repository::{NotificationError, NotificationRepository};
use std::sync::Arc;
use uuid::Uuid;

pub struct NotificationService {
    repository: Arc<dyn NotificationRepository>,
}

impl NotificationService {
    pub fn new(repository: Arc<dyn NotificationRepository>) -> Self {
        Self { repository }
    }

    pub async fn create_notification(
        &self,
        student_id: Uuid,
        offer_id: String,
        message: String,
    ) -> Result<Notification, NotificationError> {
        let notification = Notification::new_offer(student_id, offer_id, message);
        self.repository.create(&notification).await
    }

    pub async fn list_notifications(
        &self,
        student_id: Uuid,
    ) -> Result<Vec<Notification>, NotificationError> {
        self.repository.list_by_student(student_id).await
    }

    pub async fn mark_read(&self, id: Uuid) -> Result<Notification, NotificationError> {
        self.repository.mark_read(id).await
    }
}
