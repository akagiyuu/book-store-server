use std::sync::LazyLock;

use jsonwebtoken::{DecodingKey, EncodingKey};
use serde::Deserialize;

const fn default_port() -> u16 {
    3000
}

fn default_cors_domain() -> String {
    "http://localhost:3000".to_string()
}

fn default_token_cookie() -> String {
    "token".to_string()
}

fn default_jwt_secret() -> String {
    "secret".to_string()
}

const fn default_jwt_expired_in() -> u64 {
    26 * 60 * 60
}

const fn default_bcrypt_cost() -> u32 {
    10
}

const fn default_bcrypt_salt() -> [u8; 16] {
    [0; 16]
}

fn default_google_authorized_redirect_url() -> String {
    "http://localhost:3000/auth/google/authorized".to_string()
}

fn default_ollama_host() -> String {
    "http://localhost".to_string()
}

const fn default_ollama_port() -> u16 {
    11434
}

#[derive(Deserialize)]
pub struct Config {
    pub database_url: String,

    #[serde(default = "default_port")]
    pub port: u16,

    #[serde(default = "default_cors_domain")]
    pub cors_domain: String,

    #[serde(default = "default_token_cookie")]
    pub token_cookie: String,

    #[serde(default = "default_jwt_secret")]
    pub jwt_secret: String,

    #[serde(default = "default_jwt_expired_in")]
    pub jwt_expired_in: u64,

    #[serde(default = "default_bcrypt_cost")]
    pub bcrypt_cost: u32,

    #[serde(default = "default_bcrypt_salt")]
    pub bcrypt_salt: [u8; 16],

    pub google_client_id: String,

    pub google_client_secret: String,

    #[serde(default = "default_google_authorized_redirect_url")]
    pub google_authorized_redirect_url: String,

    #[serde(default = "default_ollama_host")]
    pub ollama_host: String,

    #[serde(default = "default_ollama_port")]
    pub ollama_port: u16,
}

pub static CONFIG: LazyLock<Config> = LazyLock::new(|| {
    ::config::Config::builder()
        .add_source(::config::Environment::default().try_parsing(true))
        .build()
        .unwrap()
        .try_deserialize()
        .unwrap()
});

pub struct Keys {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}

pub static KEYS: LazyLock<Keys> = LazyLock::new(|| Keys {
    encoding: EncodingKey::from_secret(CONFIG.jwt_secret.as_bytes()),
    decoding: DecodingKey::from_secret(CONFIG.jwt_secret.as_bytes()),
});
