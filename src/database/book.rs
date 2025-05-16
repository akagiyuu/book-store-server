use async_stream::try_stream;
use futures::{Stream, StreamExt};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, PgTransaction, Result};
use utoipa::ToSchema;
use uuid::Uuid;

use super::{author, category};

#[derive(Deserialize, ToSchema)]
pub struct InsertBook {
    pub isbn: String,
    pub title: String,
    pub description: String,
    pub authors: Vec<author::InsertAuthor>,
    pub categories: Vec<category::InsertCategory>,
}

async fn insert_author(
    id: Uuid,
    author: &author::InsertAuthor,
    transaction: &mut PgTransaction<'_>,
) -> Result<()> {
    let author_id = author::insert(author, &mut **transaction).await?;

    sqlx::query!(
        "INSERT INTO book_authors(book_id, author_id) VALUES($1, $2)",
        id,
        author_id
    )
    .execute(&mut **transaction)
    .await?;

    Ok(())
}

async fn insert_category(
    id: Uuid,
    category: &category::InsertCategory,
    transaction: &mut PgTransaction<'_>,
) -> Result<()> {
    let category_id = category::insert(category, &mut **transaction).await?;

    sqlx::query!(
        "INSERT INTO book_categories(book_id, category_id) VALUES($1, $2)",
        id,
        category_id
    )
    .execute(&mut **transaction)
    .await?;

    Ok(())
}

pub async fn insert(params: InsertBook, pool: &PgPool) -> Result<Uuid> {
    let mut transaction = pool.begin().await?;

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
    .await?;

    for author in &params.authors {
        insert_author(id, author, &mut transaction).await?;
    }

    for category in &params.categories {
        insert_category(id, category, &mut transaction).await?;
    }

    transaction.commit().await?;

    Ok(id)
}

#[derive(Serialize, ToSchema)]
pub struct Book {
    pub id: Uuid,
    pub isbn: String,
    pub title: String,
    pub description: String,
    pub authors: Vec<author::Author>,
    pub categories: Vec<category::Category>,
}

pub async fn get(id: Uuid, pool: &PgPool) -> Result<Book> {
    let mut transaction = pool.begin().await?;

    let book_raw = sqlx::query!(
        "SELECT id, isbn, title, description FROM books WHERE id = $1",
        id
    )
    .fetch_one(&mut *transaction)
    .await?;

    let authors = author::get_by_book_id(id, &mut *transaction).await?;
    let categories = category::get_by_book_id(id, &mut *transaction).await?;

    transaction.commit().await?;

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
            let book_raw = book_raw?;

            let mut transaction = pool.begin().await?;

            let authors = author::get_by_book_id(book_raw.id, &mut *transaction).await?;
            let categories = category::get_by_book_id(book_raw.id, &mut *transaction).await?;

            transaction.commit().await?;

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
    authors: &[author::InsertAuthor],
    transaction: &mut PgTransaction<'_>,
) -> Result<()> {
    sqlx::query!("DELETE FROM book_authors WHERE book_id = $1", id)
        .execute(&mut **transaction)
        .await?;

    for author in authors {
        insert_author(id, author, transaction).await?;
    }

    Ok(())
}

async fn update_category(
    id: Uuid,
    categories: &[category::InsertCategory],
    transaction: &mut PgTransaction<'_>,
) -> Result<()> {
    sqlx::query!("DELETE FROM book_categories WHERE book_id = $1", id)
        .execute(&mut **transaction)
        .await?;

    for category in categories {
        insert_category(id, category, transaction).await?;
    }

    Ok(())
}

#[derive(Deserialize, ToSchema)]
pub struct UpdateBook {
    pub isbn: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub authors: Option<Vec<author::InsertAuthor>>,
    pub categories: Option<Vec<category::InsertCategory>>,
}

pub async fn update(id: Uuid, params: &UpdateBook, pool: &PgPool) -> Result<()> {
    let mut transaction = pool.begin().await?;

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
    .await?;

    if let Some(authors) = &params.authors {
        update_author(id, authors, &mut transaction).await?;
    }

    if let Some(categories) = &params.categories {
        update_category(id, categories, &mut transaction).await?;
    }

    transaction.commit().await?;

    Ok(())
}

pub async fn delete(id: Uuid, database: &PgPool) -> Result<()> {
    sqlx::query!("DELETE FROM books WHERE id = $1", id)
        .execute(database)
        .await?;

    Ok(())
}
