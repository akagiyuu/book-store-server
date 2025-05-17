use serde::{Deserialize, Serialize};
use sqlx::{PgExecutor, Result};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Deserialize, ToSchema)]
pub struct InsertAuthor {
    pub name: String,
}

pub async fn insert(params: &InsertAuthor, executor: impl PgExecutor<'_>) -> Result<Uuid> {
    sqlx::query_scalar!(
        r#"
            INSERT INTO authors(name)
            VALUES ($1)
            ON CONFLICT DO NOTHING
            RETURNING id
        "#,
        params.name,
    )
    .fetch_one(executor)
    .await
}

#[derive(Serialize, ToSchema)]
pub struct Author {
    pub id: Uuid,
    pub name: String,
}

pub async fn get(id: Uuid, executor: impl PgExecutor<'_>) -> Result<Author> {
    sqlx::query_as!(Author, "SELECT id, name FROM authors WHERE id = $1", id)
        .fetch_one(executor)
        .await
}

pub async fn get_by_book_id(book_id: Uuid, executor: impl PgExecutor<'_>) -> Result<Vec<Author>> {
    sqlx::query_as!(
        Author,
        r#"
            SELECT id, name
            FROM authors
            WHERE id IN (SELECT author_id FROM book_authors WHERE id = $1)
        "#,
        book_id
    )
    .fetch_all(executor)
    .await
}

pub async fn get_all(executor: impl PgExecutor<'_>) -> Result<Vec<Author>> {
    sqlx::query_as!(Author, "SELECT id, name FROM authors",)
        .fetch_all(executor)
        .await
}

pub async fn delete(id: Uuid, executor: impl PgExecutor<'_>) -> Result<()> {
    sqlx::query!("DELETE FROM authors WHERE id = $1", id)
        .execute(executor)
        .await?;

    Ok(())
}
