use sqlx::PgPool;
use uuid::Uuid;

use crate::{Result, util::generate_random_string};

pub async fn is_existed(email: &str, database: &PgPool) -> Result<bool> {
    let is_existed =
        sqlx::query_scalar!("SELECT EXISTS(SELECT 1 FROM users WHERE email = $1)", email)
            .fetch_one(database)
            .await
            .unwrap()
            .unwrap_or(false);

    Ok(is_existed)
}

pub struct User {
    id: Uuid,
    password: String,
}

async fn get(email: &str, database: &PgPool) -> Result<User> {
    let password = sqlx::query_as!(
        User,
        "SELECT id, password FROM users WHERE email = $1 LIMIT 1",
        email
    )
    .fetch_one(database)
    .await
    .unwrap();

    Ok(password)
}

pub async fn insert(
    email: &str,
    password: Option<String>,
    first_name: &str,
    last_name: &str,
    database: &PgPool,
) -> Result<Uuid> {
    let password = password.unwrap_or_else(|| generate_random_string(10));

    let id = sqlx::query_scalar!(
        r#"
            INSERT INTO users(email, password, first_name, last_name)
            VALUES($1, $2, $3, $4)
            RETURNING id
        "#,
        email,
        password,
        first_name,
        last_name,
    )
    .fetch_one(database)
    .await
    .unwrap();

    Ok(id)
}
