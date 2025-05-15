use async_stream::try_stream;
use futures::{Stream, StreamExt};
use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

use crate::Result;

pub struct Book {
    pub id: Uuid,
    pub isbn: String,
    pub title: String,
    pub description: String,
    pub authors: Vec<String>,
    pub categories: Vec<String>,
}

async fn insert_author(
    id: Uuid,
    author_id: Uuid,
    transaction: &mut Transaction<'static, Postgres>,
) -> Result<()> {
    sqlx::query!(
        "INSERT INTO book_authors(book_id, author_id) VALUES($1, $2)",
        id,
        author_id
    )
    .execute(&mut **transaction)
    .await
    .unwrap();

    Ok(())
}

async fn insert_category(
    id: Uuid,
    category_id: Uuid,
    transaction: &mut Transaction<'static, Postgres>,
) -> Result<()> {
    sqlx::query!(
        "INSERT INTO book_categories(book_id, category_id) VALUES($1, $2)",
        id,
        category_id
    )
    .execute(&mut **transaction)
    .await
    .unwrap();

    Ok(())
}

pub async fn insert(
    isbn: &str,
    title: &str,
    description: &str,
    authors: Vec<Uuid>,
    categories: Vec<Uuid>,
    database: &PgPool,
) -> Result<Uuid> {
    let mut transaction = database.begin().await.unwrap();

    let id = sqlx::query_scalar!(
        r#"
            INSERT INTO books(isbn, title, description)
            VALUES ($1, $2, $3)
            RETURNING id
        "#,
        isbn,
        title,
        description
    )
    .fetch_one(&mut *transaction)
    .await
    .unwrap();

    for author in authors {
        insert_author(id, author, &mut transaction).await.unwrap();
    }

    for category in categories {
        insert_category(id, category, &mut transaction)
            .await
            .unwrap();
    }

    Ok(id)
}

async fn get_author(
    id: Uuid,
    transaction: &mut Transaction<'static, Postgres>,
) -> Result<Vec<String>> {
    let authors = sqlx::query_scalar!(
        r#"
            SELECT name FROM authors
            WHERE id IN (SELECT author_id FROM book_authors WHERE book_id = $1)
        "#,
        id
    )
    .fetch_all(&mut **transaction)
    .await
    .unwrap();

    Ok(authors)
}

async fn get_category(
    id: Uuid,
    transaction: &mut Transaction<'static, Postgres>,
) -> Result<Vec<String>> {
    let categories = sqlx::query_scalar!(
        r#"
            SELECT name FROM categories
            WHERE id IN (SELECT category_id FROM book_categories WHERE book_id = $1)
        "#,
        id
    )
    .fetch_all(&mut **transaction)
    .await
    .unwrap();

    Ok(categories)
}

pub async fn get(id: Uuid, database: &PgPool) -> Result<Book> {
    let mut transaction = database.begin().await.unwrap();

    let book_raw = sqlx::query!(
        "SELECT id, isbn, title, description FROM books WHERE id = $1",
        id
    )
    .fetch_one(&mut *transaction)
    .await
    .unwrap();

    let authors = get_author(id, &mut transaction).await.unwrap();
    let categories = get_category(id, &mut transaction).await.unwrap();

    transaction.commit().await.unwrap();

    Ok(Book {
        id: book_raw.id,
        isbn: book_raw.isbn,
        title: book_raw.title,
        description: book_raw.description,
        authors,
        categories,
    })
}

pub fn get_all(database: &PgPool) -> impl Stream<Item = Result<Book>> {
    let mut books_raw =
        sqlx::query!("SELECT id, isbn, title, description FROM books",).fetch(database);

    try_stream! {
        while let Some(book_raw) = books_raw.next().await {
            let book_raw = book_raw.unwrap();

            let mut transaction = database.begin().await.unwrap();

            let authors = get_author(book_raw.id, &mut transaction).await.unwrap();
            let categories = get_category(book_raw.id, &mut transaction).await.unwrap();

            transaction.commit().await.unwrap();

            yield Book {
                id: book_raw.id,
                isbn: book_raw.isbn,
                title: book_raw.title,
                description: book_raw.description,
                authors,
                categories,
            };
        }
    }
}

pub async fn update_author(id: Uuid, authors: Vec<Uuid>, database: &PgPool) -> Result<()> {
    let mut transaction = database.begin().await.unwrap();

    sqlx::query!("DELETE FROM book_authors WHERE book_id = $1", id)
        .execute(&mut *transaction)
        .await
        .unwrap();

    for author in authors {
        insert_author(id, author, &mut transaction).await.unwrap();
    }

    Ok(())
}

pub async fn update_category(id: Uuid, categories: Vec<Uuid>, database: &PgPool) -> Result<()> {
    let mut transaction = database.begin().await.unwrap();

    sqlx::query!("DELETE FROM book_categories WHERE book_id = $1", id)
        .execute(&mut *transaction)
        .await
        .unwrap();

    for category in categories {
        insert_category(id, category, &mut transaction)
            .await
            .unwrap();
    }

    Ok(())
}

pub async fn update(
    id: Uuid,
    isbn: Option<&str>,
    title: Option<&str>,
    description: Option<&str>,
    database: &PgPool,
) -> Result<()> {
    sqlx::query!(
        r#"
            UPDATE books
            SET 
                isbn = COALESCE(isbn, $2),
                title = COALESCE(title, $3),
                description = COALESCE(description, $4),
                update_at = now()
            WHERE id = $1
        "#,
        id,
        isbn,
        title,
        description
    )
    .execute(database)
    .await
    .unwrap();

    Ok(())
}

pub async fn delete(id: Uuid, database: &PgPool) -> Result<()> {
    sqlx::query!("DELETE FROM books WHERE id = $1", id)
        .execute(database)
        .await
        .unwrap();

    Ok(())
}
