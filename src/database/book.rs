use async_stream::try_stream;
use futures::{Stream, StreamExt};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, PgTransaction};
use uuid::Uuid;

use crate::Result;

use super::{author, category};

#[derive(Deserialize)]
pub struct Insert {
    pub isbn: String,
    pub title: String,
    pub description: String,
    pub authors: Vec<author::Insert>,
    pub categories: Vec<category::Insert>,
}

async fn insert_author(
    id: Uuid,
    author: &author::Insert,
    transaction: &mut PgTransaction<'_>,
) -> Result<()> {
    let author_id = author::insert(author, &mut **transaction).await.unwrap();

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
    category: &category::Insert,
    transaction: &mut PgTransaction<'_>,
) -> Result<()> {
    let category_id = category::insert(category, &mut **transaction)
        .await
        .unwrap();

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

pub async fn insert(params: Insert, pool: &PgPool) -> Result<Uuid> {
    let mut transaction = pool.begin().await.unwrap();

    let id = sqlx::query_scalar!(
        r#"
            INSERT INTO books(isbn, title, description)
            VALUES ($1, $2, $3)
            RETURNING id
        "#,
        params.isbn,
        params.title,
        params.description
    )
    .fetch_one(&mut *transaction)
    .await
    .unwrap();

    for author in &params.authors {
        insert_author(id, author, &mut transaction).await.unwrap();
    }

    for category in &params.categories {
        insert_category(id, category, &mut transaction)
            .await
            .unwrap();
    }

    transaction.commit().await.unwrap();

    Ok(id)
}

#[derive(Serialize)]
pub struct Book {
    pub id: Uuid,
    pub isbn: String,
    pub title: String,
    pub description: String,
    pub authors: Vec<author::Author>,
    pub categories: Vec<category::Category>,
}

pub async fn get(id: Uuid, pool: &PgPool) -> Result<Book> {
    let mut transaction = pool.begin().await.unwrap();

    let book_raw = sqlx::query!(
        "SELECT id, isbn, title, description FROM books WHERE id = $1",
        id
    )
    .fetch_one(&mut *transaction)
    .await
    .unwrap();

    let authors = author::get_by_book_id(id, &mut *transaction).await?;
    let categories = category::get_by_book_id(id, &mut *transaction)
        .await
        .unwrap();

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

pub fn get_all(pool: &PgPool) -> impl Stream<Item = Result<Book>> {
    let mut books_raw = sqlx::query!("SELECT id, isbn, title, description FROM books",).fetch(pool);

    try_stream! {
        while let Some(book_raw) = books_raw.next().await {
            let book_raw = book_raw.unwrap();

            let mut transaction = pool.begin().await.unwrap();

            let authors = author::get_by_book_id(book_raw.id, &mut *transaction).await?;
            let categories = category::get_by_book_id(book_raw.id, &mut *transaction)
                .await
                .unwrap();

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

async fn update_author(
    id: Uuid,
    authors: &[author::Insert],
    transaction: &mut PgTransaction<'_>,
) -> Result<()> {
    sqlx::query!("DELETE FROM book_authors WHERE book_id = $1", id)
        .execute(&mut **transaction)
        .await
        .unwrap();

    for author in authors {
        insert_author(id, author, transaction).await.unwrap();
    }

    Ok(())
}

async fn update_category(
    id: Uuid,
    categories: &[category::Insert],
    transaction: &mut PgTransaction<'_>,
) -> Result<()> {
    sqlx::query!("DELETE FROM book_categories WHERE book_id = $1", id)
        .execute(&mut **transaction)
        .await
        .unwrap();

    for category in categories {
        insert_category(id, category, transaction).await.unwrap();
    }

    Ok(())
}

#[derive(Deserialize)]
pub struct Update {
    pub isbn: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub authors: Option<Vec<author::Insert>>,
    pub categories: Option<Vec<category::Insert>>,
}

pub async fn update(id: Uuid, params: &Update, pool: &PgPool) -> Result<()> {
    let mut transaction = pool.begin().await.unwrap();

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
        params.isbn,
        params.title,
        params.description
    )
    .execute(&mut *transaction)
    .await
    .unwrap();

    if let Some(authors) = &params.authors {
        update_author(id, authors, &mut transaction).await.unwrap();
    }

    if let Some(categories) = &params.categories {
        update_category(id, categories, &mut transaction)
            .await
            .unwrap();
    }

    transaction.commit().await.unwrap();

    Ok(())
}

pub async fn delete(id: Uuid, database: &PgPool) -> Result<()> {
    sqlx::query!("DELETE FROM books WHERE id = $1", id)
        .execute(database)
        .await
        .unwrap();

    Ok(())
}
