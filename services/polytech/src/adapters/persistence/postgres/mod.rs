use crate::domain::student::Student;
use crate::ports::student_repository::{StudentError, StudentRepository};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone)]
pub struct PostgresStudentRepository {
    pool: PgPool,
}

impl PostgresStudentRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl StudentRepository for PostgresStudentRepository {
    async fn create(&self, student: &Student) -> Result<Student, StudentError> {
        sqlx::query_as::<_, Student>(
            r#"
            INSERT INTO students (id, firstname, name, domain)
            VALUES ($1, $2, $3, $4)
            RETURNING id, firstname, name, domain
            "#,
        )
        .bind(student.id)
        .bind(&student.firstname)
        .bind(&student.name)
        .bind(&student.domain)
        .fetch_one(&self.pool)
        .await
        .map_err(|e: sqlx::Error| StudentError::DatabaseError(e.to_string()))
    }

    async fn get(&self, id: Uuid) -> Result<Student, StudentError> {
        sqlx::query_as::<_, Student>(
            r#"
            SELECT id, firstname, name, domain
            FROM students
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e: sqlx::Error| StudentError::DatabaseError(e.to_string()))?
        .ok_or(StudentError::NotFound)
    }

    async fn list_by_domain(&self, domain: &str) -> Result<Vec<Student>, StudentError> {
        sqlx::query_as::<_, Student>(
            r#"
            SELECT id, firstname, name, domain
            FROM students
            WHERE domain = $1
            "#,
        )
        .bind(domain)
        .fetch_all(&self.pool)
        .await
        .map_err(|e: sqlx::Error| StudentError::DatabaseError(e.to_string()))
    }

    async fn update(&self, id: Uuid, student_data: Student) -> Result<Student, StudentError> {
        sqlx::query_as::<_, Student>(
            r#"
            UPDATE students
            SET firstname = $1, name = $2, domain = $3
            WHERE id = $4
            RETURNING id, firstname, name, domain
            "#,
        )
        .bind(&student_data.firstname)
        .bind(&student_data.name)
        .bind(&student_data.domain)
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e: sqlx::Error| StudentError::DatabaseError(e.to_string()))?
        .ok_or(StudentError::NotFound)
    }

    async fn delete(&self, id: Uuid) -> Result<(), StudentError> {
        let result = sqlx::query(
            r#"
            DELETE FROM students
            WHERE id = $1
            "#,
        )
        .bind(id)
        .execute(&self.pool)
        .await
        .map_err(|e: sqlx::Error| StudentError::DatabaseError(e.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(StudentError::NotFound);
        }

        Ok(())
    }
}
