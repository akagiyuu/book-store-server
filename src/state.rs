use oauth2::{AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl, basic::BasicClient};
use sqlx::PgPool;

use crate::{Result, config::CONFIG};

#[derive(Debug)]
pub struct ApiState {
    pub database: PgPool,
    pub google_oauth_client: BasicClient,
    pub http_client: reqwest::Client,
}

impl ApiState {
    pub async fn new() -> Result<Self> {
        let database = PgPool::connect(&CONFIG.database_url).await.unwrap();

        let google_oauth_client = BasicClient::new(
            ClientId::new(CONFIG.google_client_id.clone()),
            Some(ClientSecret::new(CONFIG.google_client_secret.clone())),
            AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string()).unwrap(),
            Some(TokenUrl::new("https://www.googleapis.com/oauth2/v3/token".to_string()).unwrap()),
        )
        .set_redirect_uri(RedirectUrl::new(CONFIG.google_authorized_redirect_url.clone()).unwrap());

        let http_client = reqwest::ClientBuilder::new()
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .unwrap();

        Ok(Self {
            database,
            google_oauth_client,
            http_client,
        })
    }
}
