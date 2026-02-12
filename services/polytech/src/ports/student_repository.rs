use crate::domain::student::Student;

use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum StudentError {
    #[error("Student not found")]
    NotFound,
    #[error("Database error: {0}")]
    DatabaseError(String),
    #[error("Invalid data: {0}")]
    InvalidData(String),
}

pub trait StudentRepository: Send + Sync {
    fn create(
        &self,
        student: &Student,
    ) -> impl std::future::Future<Output = Result<Student, StudentError>> + Send;
    fn get(
        &self,
        id: Uuid,
    ) -> impl std::future::Future<Output = Result<Student, StudentError>> + Send;
    fn list_by_domain(
        &self,
        domain: &str,
    ) -> impl std::future::Future<Output = Result<Vec<Student>, StudentError>> + Send;
    fn update(
        &self,
        id: Uuid,
        student_data: Student,
    ) -> impl std::future::Future<Output = Result<Student, StudentError>> + Send;
    fn delete(
        &self,
        id: Uuid,
    ) -> impl std::future::Future<Output = Result<(), StudentError>> + Send;
}
