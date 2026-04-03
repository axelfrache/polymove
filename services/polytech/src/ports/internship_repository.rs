use crate::domain::internship::Internship;
use futures::future::BoxFuture;
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum InternshipError {
    #[error("Internship not found")]
    NotFound,
    #[error("Database error: {0}")]
    DatabaseError(String),
}

pub trait InternshipRepository: Send + Sync {
    fn save<'a>(
        &'a self,
        internship: &'a Internship,
    ) -> BoxFuture<'a, Result<Internship, InternshipError>>;
    fn get(&self, id: Uuid) -> BoxFuture<'_, Result<Internship, InternshipError>>;
    fn list_by_student(
        &self,
        student_id: Uuid,
    ) -> BoxFuture<'_, Result<Vec<Internship>, InternshipError>>;
}
