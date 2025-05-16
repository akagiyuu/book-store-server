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

pub struct Review {
    pub book_id: Uuid,
    pub user_id: Uuid,
    pub rate: f32,
    pub content: String,
}

impl Review {
    pub async fn get(book_id: Uuid, user_id: Uuid, executor: impl PgExecutor<'_>) -> Result<Self> {
        let review = sqlx::query_as!(
            Self,
            r#"SELECT book_id, user_id, rate, content FROM reviews WHERE book_id = $1 AND user_id = $2"#,
            book_id,
            user_id
        )
        .fetch_one(executor)
        .await
        .unwrap();

        Ok(review)
    }

    pub async fn get_all(executor: impl PgExecutor<'_>) -> Result<Vec<Self>> {
        let reviews = sqlx::query_as!(
            Self,
            r#"SELECT book_id, user_id, rate, content FROM reviews"#,
        )
        .fetch_all(executor)
        .await
        .unwrap();

        Ok(reviews)
    }
}
