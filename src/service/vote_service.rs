// Copyright (c) 2018 Ba Hai Phan
// Code released under the MIT license
// https://opensource.org/licenses/mit-license.php

use crate::{
    models::{
        vote::Vote,
        answer::Answer
    },
    utils::*,
    constants,
    config::db::Pool,
    errors::{ ServiceError, StatusCode }
};
use sql_client::vote_client::VoteClient;
use actix_web::http::header::HeaderValue;

pub async fn get_votes_by_user_and_theme(user_id: &str, theme_id: i32, pool: &Pool) -> Result<Vec<Vote>, ServiceError> {
    pool.get_votes_by_user_and_theme(user_id, theme_id).await.map_err(|_| 
        ServiceError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            constants::MESSAGE_INTERNAL_SERVER_ERROR.to_string()
        )
    )
}

pub async fn summarize_result(theme_id: i32, pool: &Pool) -> Result<Vec<Answer>, ServiceError> {
    pool.summarize_result(theme_id).await.map_err(|_| 
        ServiceError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            constants::MESSAGE_INTERNAL_SERVER_ERROR.to_string()
        )
    )
}

pub async fn post_votes(
    authen_header: &HeaderValue,
    user_id: &str,
    theme_id: i32,
    votes: Vec<Vote>,
    pool: &Pool
) -> Result<(), ServiceError> {
    let error = |_| ServiceError::new(
        StatusCode::INTERNAL_SERVER_ERROR,
        constants::MESSAGE_PROCESS_TOKEN_ERROR.to_string()
    );
    let authen_str = authen_header.to_str().map_err(|_| ()).map_err(error)?;
    if !authen_str.starts_with("bearer") { return Err(error(()));}
    let token = authen_str[6..authen_str.len()].trim();
    let token_data = decode_token(token.to_string()).map_err(|_| ()).map_err(error)?.claims;
    if token_data.user_id != user_id {
        return Err(
            ServiceError::new(
                StatusCode::BAD_REQUEST,
                constants::MESSAGE_INVALID_TOKEN.to_string()
            )
        )
    }
    pool.post_votes(user_id, theme_id, votes).await.map_err(|_| 
        ServiceError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            constants::MESSAGE_INTERNAL_SERVER_ERROR.to_string()
        )
    )
}