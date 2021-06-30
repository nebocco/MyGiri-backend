// Copyright (c) 2018 Ba Hai Phan
// Code released under the MIT license
// https://opensource.org/licenses/mit-license.php

use once_cell::sync::Lazy;
use serde::{ Serialize, Deserialize };
use jsonwebtoken::{
    EncodingKey,
    Header
};

use crate::models::user::User;

pub static ENCODE_KEY: Lazy<String> = Lazy::new(|| 
    std::env::var("ENCODE_KEY").unwrap_or("nebo".repeat(4))
);
static ONE_WEEK: i64 = 60 * 60 * 24 * 7; // in seconds

#[derive(Serialize)]
pub struct LoginDTO {
    pub user: User,
    pub login_session: String,
}

#[derive(Serialize)]
pub struct TokenBodyResponse {
    user: User,
    token: String,
    token_type: String
}

impl TokenBodyResponse {
    pub fn new(login_dto: &LoginDTO) -> Self {
        Self {
            user: login_dto.user.clone(),
            token: UserToken::generate_token(login_dto),
            token_type: "bearer".to_string()
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct UserToken {
    pub iat: i64, // issued at
    pub exp: i64, // expire at
    pub user_id: String,
    pub login_session: String,
}

impl UserToken {
    pub fn generate_token(login: &LoginDTO) -> String {
        let now = chrono::Local::now().timestamp(); 
        let payload = UserToken {
            iat: now,
            exp: now + ONE_WEEK,
            user_id: login.user.user_id.clone(),
            login_session: login.login_session.clone(),
        };

        jsonwebtoken::encode(
            &Header::default(),
            &payload,
            &EncodingKey::from_secret(ENCODE_KEY.as_bytes())
        ).unwrap()
    }
}