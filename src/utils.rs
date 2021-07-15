// Copyright (c) 2018 Ba Hai Phan
// Code released under the MIT license
// https://opensource.org/licenses/mit-license.php

use once_cell::sync::Lazy;
use argon2::{self, Config};
use crate::models::login::{UserToken, ENCODE_KEY};
use crate::config::db::Pool;
use crate::service::auth_service::is_valid_login_session;
use jsonwebtoken::{TokenData, DecodingKey, Validation};

pub static SECRET_KEY: Lazy<String> = Lazy::new(||
    std::env::var("SECRET_KEY").unwrap_or_else(|_| "nebo".repeat(8))
);

const SALT: &'static [u8] = b"saltyfish";

// WARNING THIS IS ONLY FOR DEMO PLEASE DO MORE RESEARCH FOR PRODUCTION USE
pub fn hash_password(password: &str) -> Result<String, String> {
    let config = Config {
        secret: SECRET_KEY.as_bytes(),
        ..Default::default()
    };
    let hash = argon2::hash_encoded(password.as_bytes(), &SALT, &config)
        .map_err(|_| "argon error".to_string())?;
    Ok(hash)
}

pub fn verify_password(hash: &str, password: &str) -> Result<bool, String> {
    let is_valid = argon2::verify_encoded_ext(
        hash,
        password.as_bytes(),
        SECRET_KEY.as_bytes(),
        &[]
    ).map_err(|_| "argon error".to_string())?;
    Ok(is_valid)
}

pub fn decode_token(token: String) -> jsonwebtoken::errors::Result<TokenData<UserToken>> {
    jsonwebtoken::decode::<UserToken>(&token, &DecodingKey::from_secret(ENCODE_KEY.as_bytes()), &Validation::default())
}

pub async fn verify_token(token_data: &TokenData<UserToken>, pool: &Pool) -> Result<String, String> {
    match is_valid_login_session(&token_data.claims, pool).await {
        Ok(()) => Ok(token_data.claims.user_id.to_string()),
        Err(message) => Err(message)
    }
}