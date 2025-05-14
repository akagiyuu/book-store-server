use oauth2::{
    AuthUrl, Client, ClientId, ClientSecret, EmptyExtraTokenFields, EndpointNotSet, EndpointSet,
    RedirectUrl, RevocationErrorResponseType, StandardErrorResponse, StandardRevocableToken,
    StandardTokenIntrospectionResponse, StandardTokenResponse, TokenUrl,
    basic::{BasicClient, BasicErrorResponseType, BasicTokenType},
};
use sqlx::PgPool;

use crate::{Result, config::CONFIG};

type GoogleOAuthClient = Client<
    StandardErrorResponse<BasicErrorResponseType>,
    StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>,
    StandardTokenIntrospectionResponse<EmptyExtraTokenFields, BasicTokenType>,
    StandardRevocableToken,
    StandardErrorResponse<RevocationErrorResponseType>,
    EndpointSet,
    EndpointNotSet,
    EndpointNotSet,
    EndpointNotSet,
    EndpointSet,
>;

#[derive(Debug)]
pub struct ApiState {
    pub database: PgPool,
    pub google_oauth_client: GoogleOAuthClient,
}

impl ApiState {
    pub async fn new() -> Result<Self> {
        let database = PgPool::connect(&CONFIG.database_url).await.unwrap();

        let google_oauth_client = BasicClient::new(ClientId::new(CONFIG.google_client_id.clone()))
            .set_client_secret(ClientSecret::new(CONFIG.google_client_secret.clone()))
            .set_auth_uri(
                AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string()).unwrap(),
            )
            .set_token_uri(
                TokenUrl::new("https://www.googleapis.com/oauth2/v3/token".to_string()).unwrap(),
            )
            .set_redirect_uri(
                RedirectUrl::new(CONFIG.google_authorized_redirect_url.clone()).unwrap(),
            );

        Ok(Self {
            database,
            google_oauth_client,
        })
    }
}
