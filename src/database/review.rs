use serde::{Deserialize, Serialize};
use sqlx::{PgExecutor, Result};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Deserialize, ToSchema)]
pub struct InsertReview {
    pub rate: f32,
    pub content: String,
}

pub async fn insert(
    book_id: Uuid,
    user_id: Uuid,
    params: &InsertReview,
    executor: impl PgExecutor<'_>,
) -> Result<()> {
    sqlx::query!(
        r#"
            INSERT INTO reviews(book_id, user_id, rate, content)
            VALUES ($1, $2, $3, $4)
        "#,
        book_id,
        user_id,
        params.rate,
        params.content,
    )
    .execute(executor)
    .await?;

    Ok(())
}

#[derive(Serialize, ToSchema)]
pub struct Review {
    pub book_id: Uuid,
    pub user_email: String,
    pub rate: f32,
    pub content: String,
}

pub async fn get(id: Uuid, executor: impl PgExecutor<'_>) -> Result<Review> {
    sqlx::query_as!(
        Review,
        r#"
            SELECT book_id, users.email as user_email, rate, content
            FROM reviews
            INNER JOIN users ON users.id = reviews.user_id
            WHERE reviews.id = $1
        "#,
        id
    )
    .fetch_one(executor)
    .await
}

pub async fn get_by_book(book_id: Uuid, executor: impl PgExecutor<'_>) -> Result<Vec<Review>> {
    sqlx::query_as!(
        Review,
        r#"
            SELECT book_id, users.email as user_email, rate, content
            FROM reviews
            INNER JOIN users ON users.id = reviews.user_id
            WHERE book_id = $1
        "#,
        book_id
    )
    .fetch_all(executor)
    .await
}

#[derive(Deserialize, ToSchema)]
pub struct UpdateReview {
    pub rate: Option<f32>,
    pub content: Option<String>,
}

pub async fn update(
    id: Uuid,
    user_id: Uuid,
    params: &UpdateReview,
    executor: impl PgExecutor<'_>,
) -> Result<()> {
    sqlx::query!(
        r#"
            UPDATE reviews
            SET 
                rate = COALESCE(rate, $3),
                content = COALESCE(content, $4),
                update_at = now()
            WHERE id = $1 AND user_id = $2
        "#,
        id,
        user_id,
        params.rate,
        params.content
    )
    .execute(executor)
    .await?;

    Ok(())
}

pub async fn delete(id: Uuid, user_id: Uuid, executor: impl PgExecutor<'_>) -> Result<()> {
    sqlx::query!(
        "DELETE FROM reviews WHERE id = $1 AND user_id = $2",
        id,
        user_id
    )
    .execute(executor)
    .await?;

    Ok(())
}
