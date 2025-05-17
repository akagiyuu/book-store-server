use serde::{Deserialize, Serialize};
use sqlx::{PgExecutor, Result};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Deserialize, ToSchema)]
pub struct InsertCategory {
    pub name: String,
    pub description: String,
}

pub async fn insert(params: &InsertCategory, executor: impl PgExecutor<'_>) -> Result<Uuid> {
    sqlx::query_scalar!(
        r#"
            INSERT INTO categories(name, description)
            VALUES ($1, $2)
            ON CONFLICT DO NOTHING
            RETURNING id
        "#,
        params.name,
        params.description
    )
    .fetch_one(executor)
    .await
}

#[derive(Serialize, ToSchema)]
pub struct Category {
    pub id: Uuid,
    pub name: String,
    pub description: String,
}

pub async fn get(id: Uuid, executor: impl PgExecutor<'_>) -> Result<Category> {
    sqlx::query_as!(
        Category,
        "SELECT id, name, description FROM categories WHERE id = $1",
        id
    )
    .fetch_one(executor)
    .await
}

pub async fn get_by_book_id(book_id: Uuid, executor: impl PgExecutor<'_>) -> Result<Vec<Category>> {
    sqlx::query_as!(
        Category,
        r#"
            SELECT id, name, description
            FROM categories
            WHERE id IN (SELECT category_id FROM book_categories WHERE id = $1)
        "#,
        book_id
    )
    .fetch_all(executor)
    .await
}

pub async fn get_all(executor: impl PgExecutor<'_>) -> Result<Vec<Category>> {
    sqlx::query_as!(Category, "SELECT id, name, description FROM categories",)
        .fetch_all(executor)
        .await
}

#[derive(Deserialize, ToSchema)]
pub struct UpdateCategory {
    pub description: Option<String>,
}

pub async fn update(
    id: Uuid,
    params: &UpdateCategory,
    executor: impl PgExecutor<'_>,
) -> Result<()> {
    sqlx::query!(
        r#"
            UPDATE categories
            SET
                description = COALESCE($2, description),
                update_at = now()
            WHERE id = $1
        "#,
        id,
        params.description
    )
    .execute(executor)
    .await?;

    Ok(())
}

pub async fn delete(id: Uuid, executor: impl PgExecutor<'_>) -> Result<()> {
    sqlx::query!("DELETE FROM categories WHERE id = $1", id)
        .execute(executor)
        .await?;

    Ok(())
}
