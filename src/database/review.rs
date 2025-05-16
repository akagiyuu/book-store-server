use sqlx::PgExecutor;
use uuid::Uuid;

use crate::Result;

pub struct InsertReview {
    pub book_id: Uuid,
    pub user_id: Uuid,
    pub rate: f32,
    pub content: String,
}

impl InsertReview {
    pub async fn insert(&self, executor: impl PgExecutor<'_>) -> Result<()> {
        sqlx::query!(
            r#"
                INSERT INTO reviews(book_id, user_id, rate, content)
                VALUES ($1, $2, $3, $4)
            "#,
            self.book_id,
            self.user_id,
            self.rate,
            self.content,
        )
        .execute(executor)
        .await
        .unwrap();

        Ok(())
    }
}
