use sqlx::PgExecutor;
use uuid::Uuid;

use crate::Result;

pub struct Insert {
    pub name: String,
}

pub async fn insert(params: &Insert, executor: impl PgExecutor<'_>) -> Result<Uuid> {
    let id = sqlx::query_scalar!(
        r#"
            INSERT INTO authors(name)
            VALUES ($1)
            RETURNING id
        "#,
        params.name,
    )
    .fetch_one(executor)
    .await
    .unwrap();

    Ok(id)
}

pub struct Author {
    pub id: Uuid,
    pub name: String,
}

pub async fn get(id: Uuid, executor: impl PgExecutor<'_>) -> Result<Author> {
    let author = sqlx::query_as!(Author, "SELECT id, name FROM authors WHERE id = $1", id)
        .fetch_one(executor)
        .await
        .unwrap();

    Ok(author)
}

pub async fn get_by_book_id(book_id: Uuid, executor: impl PgExecutor<'_>) -> Result<Vec<Author>> {
    let authors = sqlx::query_as!(
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
    .unwrap();

    Ok(authors)
}

pub async fn get_all(executor: impl PgExecutor<'_>) -> Result<Vec<Author>> {
    let author = sqlx::query_as!(Author, "SELECT id, name FROM authors",)
        .fetch_all(executor)
        .await
        .unwrap();

    Ok(author)
}

pub struct Update {
    name: Option<String>,
}

pub async fn update(id: Uuid, params: &Update, executor: impl PgExecutor<'_>) -> Result<()> {
    sqlx::query!(
        r#"
            UPDATE authors
            SET 
                name = COALESCE(name, $2),
                update_at = now()
            WHERE id = $1
        "#,
        id,
        params.name,
    )
    .execute(executor)
    .await
    .unwrap();

    Ok(())
}

pub async fn delete(id: Uuid, executor: impl PgExecutor<'_>) -> Result<()> {
    sqlx::query!("DELETE FROM authors WHERE id = $1", id)
        .execute(executor)
        .await
        .unwrap();

    Ok(())
}
