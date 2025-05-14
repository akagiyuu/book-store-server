use sqlx::PgPool;

use crate::{Result, config::CONFIG};

#[derive(Debug)]
pub struct ApiState {
    database: PgPool,
}

impl ApiState {
    pub async fn new() -> Result<Self> {
        let database = PgPool::connect(&CONFIG.database_url)
            .await
            .map_err(anyhow::Error::from)?;

        Ok(Self { database })
    }
}
