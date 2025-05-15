use sqlx::PgPool;
use uuid::Uuid;

use crate::Result;

pub struct Author {
    pub id: Uuid,
    pub name: String,
}

pub async fn insert(name: &str, database: &PgPool) -> Result<Uuid> {
    let id = sqlx::query_scalar!(
        r#"
            INSERT INTO authors(name)
            VALUES ($1)
            RETURNING id
        "#,
        name,
    )
    .fetch_one(database)
    .await
    .unwrap();

    Ok(id)
}

pub async fn get(id: Uuid, database: &PgPool) -> Result<Author> {
    let category = sqlx::query_as!(Author, "SELECT id, name FROM authors WHERE id = $1", id)
        .fetch_one(database)
        .await
        .unwrap();

    Ok(category)
}

pub async fn get_all(database: &PgPool) -> Result<Vec<Author>> {
    let category = sqlx::query_as!(Author, "SELECT id, name FROM authors",)
        .fetch_all(database)
        .await
        .unwrap();

    Ok(category)
}

pub async fn update(id: Uuid, name: Option<&str>, database: &PgPool) -> Result<()> {
    sqlx::query!(
        r#"
            UPDATE authors
            SET name = COALESCE(name, $2)
            WHERE id = $1
        "#,
        id,
        name,
    )
    .execute(database)
    .await
    .unwrap();

    Ok(())
}

pub async fn delete(id: Uuid, database: &PgPool) -> Result<()> {
    sqlx::query!("DELETE FROM authors WHERE id = $1", id)
        .execute(database)
        .await
        .unwrap();

    Ok(())
}
