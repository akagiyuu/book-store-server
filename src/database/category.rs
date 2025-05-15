use sqlx::PgPool;
use uuid::Uuid;

use crate::Result;

pub struct Category {
    pub id: Uuid,
    pub name: String,
    pub description: String,
}

pub async fn insert(name: &str, description: &str, database: &PgPool) -> Result<Uuid> {
    let id = sqlx::query_scalar!(
        r#"
            INSERT INTO categories(name, description)
            VALUES ($1, $2)
            RETURNING id
        "#,
        name,
        description
    )
    .fetch_one(database)
    .await
    .unwrap();

    Ok(id)
}

pub async fn get(id: Uuid, database: &PgPool) -> Result<Category> {
    let category = sqlx::query_as!(
        Category,
        "SELECT id, name, description FROM categories WHERE id = $1",
        id
    )
    .fetch_one(database)
    .await
    .unwrap();

    Ok(category)
}

pub async fn get_all(database: &PgPool) -> Result<Vec<Category>> {
    let category = sqlx::query_as!(Category, "SELECT id, name, description FROM categories",)
        .fetch_all(database)
        .await
        .unwrap();

    Ok(category)
}

pub async fn update(
    id: Uuid,
    name: Option<&str>,
    description: Option<&str>,
    database: &PgPool,
) -> Result<()> {
    sqlx::query!(
        r#"
            UPDATE categories
            SET
                name = COALESCE(name, $2),
                description = COALESCE(description, $3)
            WHERE id = $1
        "#,
        id,
        name,
        description
    )
    .execute(database)
    .await
    .unwrap();

    Ok(())
}

pub async fn delete(id: Uuid, database: &PgPool) -> Result<()> {
    sqlx::query!("DELETE FROM categories WHERE id = $1", id)
        .execute(database)
        .await
        .unwrap();

    Ok(())
}
