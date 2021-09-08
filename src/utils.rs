// Copyright (c) 2018 Ba Hai Phan
// Code released under the MIT license
// https://opensource.org/licenses/mit-license.php

use once_cell::sync::Lazy;
use actix_web::http::Method;
use argon2::{self, Config};
use crate::models::login::{UserToken, ENCODE_KEY};
use crate::config::db::Pool;
use crate::service::auth_service::is_valid_login_session;
use crate::constants;
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

pub fn is_ignorable(path: &str, method: &Method) -> bool {
    // ignorable request
    *method == Method::OPTIONS ||
    // ignorable routes
    constants::IGNORE_ROUTES.iter().any(|ignore_route|
        path.starts_with(ignore_route)
    ) ||
    // theme itself or theme result
    *method == Method::GET
    && path.starts_with("/api/theme/")
    && !path.contains("vote")
}

#[cfg(test)]
mod test {
    #[test]
    fn test_is_ignorable() {
        use super::is_ignorable;
        use actix_web::http::Method;

        assert!(is_ignorable("/api/theme", &Method::OPTIONS));

        assert!(is_ignorable("/api/auth/login", &Method::POST));
        assert!(is_ignorable("/api/auth/signup", &Method::POST));
        assert!(is_ignorable("/api/themes/active", &Method::GET));
        assert!(is_ignorable("/api/themes/date", &Method::GET));

        assert!(is_ignorable("/api/theme/100", &Method::GET));
        assert!(is_ignorable("/api/theme/100/result", &Method::GET));
        

        assert!(!is_ignorable("/api/theme", &Method::POST));
        assert!(!is_ignorable("/api/theme/100", &Method::POST));
        assert!(!is_ignorable("/api/theme/100/vote", &Method::POST));
        assert!(!is_ignorable("/api/theme/100/vote/user1", &Method::GET));

        assert!(!is_ignorable("/api/themes/user/user1", &Method::GET));
        assert!(!is_ignorable("/api/themes/recent/user1", &Method::GET));
    }
}