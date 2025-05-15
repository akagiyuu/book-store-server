use sqlx::PgExecutor;
use uuid::Uuid;

use crate::Result;

pub struct Insert {
    pub name: String,
    pub description: String,
}

pub async fn insert(params: &Insert, executor: impl PgExecutor<'_>) -> Result<Uuid> {
    let id = sqlx::query_scalar!(
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
    .unwrap();

    Ok(id)
}

pub struct Category {
    pub id: Uuid,
    pub name: String,
    pub description: String,
}

pub async fn get(id: Uuid, executor: impl PgExecutor<'_>) -> Result<Category> {
    let category = sqlx::query_as!(
        Category,
        "SELECT id, name, description FROM categories WHERE id = $1",
        id
    )
    .fetch_one(executor)
    .await
    .unwrap();

    Ok(category)
}

pub async fn get_by_book_id(book_id: Uuid, executor: impl PgExecutor<'_>) -> Result<Vec<Category>> {
    let authors = sqlx::query_as!(
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
    .unwrap();

    Ok(authors)
}

pub async fn get_all(executor: impl PgExecutor<'_>) -> Result<Vec<Category>> {
    let category = sqlx::query_as!(Category, "SELECT id, name, description FROM categories",)
        .fetch_all(executor)
        .await
        .unwrap();

    Ok(category)
}

pub struct Update {
    pub name: Option<String>,
    pub description: Option<String>,
}

pub async fn update(id: Uuid, params: &Update, executor: impl PgExecutor<'_>) -> Result<()> {
    sqlx::query!(
        r#"
            UPDATE categories
            SET
                name = COALESCE(name, $2),
                description = COALESCE(description, $3),
                update_at = now()
            WHERE id = $1
        "#,
        id,
        params.name,
        params.description
    )
    .execute(executor)
    .await
    .unwrap();

    Ok(())
}

pub async fn delete(id: Uuid, executor: impl PgExecutor<'_>) -> Result<()> {
    sqlx::query!("DELETE FROM categories WHERE id = $1", id)
        .execute(executor)
        .await
        .unwrap();

    Ok(())
}
