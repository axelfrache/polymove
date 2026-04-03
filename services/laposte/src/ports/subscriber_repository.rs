use crate::domain::subscriber::Subscriber;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SubscriberError {
    #[error("Database error: {0}")]
    DatabaseError(String),
    #[error("Subscriber not found")]
    NotFound,
}

pub trait SubscriberRepository: Send + Sync {
    fn create(
        &self,
        subscriber: &Subscriber,
    ) -> impl std::future::Future<Output = Result<Subscriber, SubscriberError>> + Send;

    fn get(
        &self,
        student_id: &str,
    ) -> impl std::future::Future<Output = Result<Option<Subscriber>, SubscriberError>> + Send;

    fn update(
        &self,
        subscriber: &Subscriber,
    ) -> impl std::future::Future<Output = Result<Subscriber, SubscriberError>> + Send;

    fn list_by_domain(
        &self,
        domain: &str,
    ) -> impl std::future::Future<Output = Result<Vec<Subscriber>, SubscriberError>> + Send;

    fn delete(
        &self,
        student_id: &str,
    ) -> impl std::future::Future<Output = Result<(), SubscriberError>> + Send;
}
