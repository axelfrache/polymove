use crate::domain::student::Student;
use crate::ports::student_repository::{StudentError, StudentRepository};
use uuid::Uuid;

pub struct StudentService<R: StudentRepository> {
    repository: R,
}

impl<R: StudentRepository> StudentService<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn create_student(
        &self,
        firstname: String,
        name: String,
        domain: String,
    ) -> Result<Student, StudentError> {
        let student = Student::new(firstname, name, domain);
        self.repository.create(&student).await
    }

    pub async fn get_student(&self, id: Uuid) -> Result<Student, StudentError> {
        self.repository.get(id).await
    }

    pub async fn list_students_by_domain(
        &self,
        domain: &str,
    ) -> Result<Vec<Student>, StudentError> {
        self.repository.list_by_domain(domain).await
    }

    pub async fn update_student(
        &self,
        id: Uuid,
        firstname: Option<String>,
        name: Option<String>,
        domain: Option<String>,
    ) -> Result<Student, StudentError> {
        let mut current_student = self.repository.get(id).await?;

        if let Some(f) = firstname {
            current_student.firstname = f;
        }
        if let Some(n) = name {
            current_student.name = n;
        }
        if let Some(d) = domain {
            current_student.domain = d;
        }

        self.repository.update(id, current_student).await
    }

    pub async fn delete_student(&self, id: Uuid) -> Result<(), StudentError> {
        self.repository.delete(id).await
    }
}
