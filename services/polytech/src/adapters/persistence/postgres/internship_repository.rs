use crate::domain::internship::Internship;
use crate::ports::internship_repository::{InternshipError, InternshipRepository};
use futures::future::BoxFuture;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone)]
pub struct PostgresInternshipRepository {
    pool: PgPool,
}

impl PostgresInternshipRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl InternshipRepository for PostgresInternshipRepository {
    fn save<'a>(
        &'a self,
        internship: &'a Internship,
    ) -> BoxFuture<'a, Result<Internship, InternshipError>> {
        Box::pin(async move {
            sqlx::query_as::<_, Internship>(
                r#"
                INSERT INTO internships (id, student_id, offer_id, approved, message)
                VALUES ($1, $2, $3, $4, $5)
                RETURNING id, student_id, offer_id, approved, message
                "#,
            )
            .bind(internship.id)
            .bind(internship.student_id)
            .bind(&internship.offer_id)
            .bind(internship.approved)
            .bind(&internship.message)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| InternshipError::DatabaseError(e.to_string()))
        })
    }

    fn get(&self, id: Uuid) -> BoxFuture<'_, Result<Internship, InternshipError>> {
        Box::pin(async move {
            sqlx::query_as::<_, Internship>(
                r#"
                SELECT id, student_id, offer_id, approved, message
                FROM internships
                WHERE id = $1
                "#,
            )
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| InternshipError::DatabaseError(e.to_string()))?
            .ok_or(InternshipError::NotFound)
        })
    }

    fn list_by_student(
        &self,
        student_id: Uuid,
    ) -> BoxFuture<'_, Result<Vec<Internship>, InternshipError>> {
        Box::pin(async move {
            sqlx::query_as::<_, Internship>(
                r#"
                SELECT id, student_id, offer_id, approved, message
                FROM internships
                WHERE student_id = $1
                ORDER BY id DESC
                "#,
            )
            .bind(student_id)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| InternshipError::DatabaseError(e.to_string()))
        })
    }
}
