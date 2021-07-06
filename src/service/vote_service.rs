// Copyright (c) 2018 Ba Hai Phan
// Code released under the MIT license
// https://opensource.org/licenses/mit-license.php

use crate::{
    models::vote::{ Vote, VoteResult },
    constants,
    config::db::Pool,
    errors::{ ServiceError, StatusCode }
};
use sql_client::vote_client::VoteClient;

pub async fn get_votes_by_user_and_theme(user_id: &str, theme_id: i32, pool: &Pool) -> Result<Vec<Vote>, ServiceError> {
    pool.get_votes_by_user_and_theme(user_id, theme_id).await.map_err(|_| 
        ServiceError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            constants::MESSAGE_INTERNAL_SERVER_ERROR.to_string()
        )
    )
}

pub async fn summarize_result(theme_id: i32, pool: &Pool) -> Result<Vec<VoteResult>, ServiceError> {
    pool.summarize_result(theme_id).await.map_err(|_| 
        ServiceError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            constants::MESSAGE_INTERNAL_SERVER_ERROR.to_string()
        )
    )
}

pub async fn post_votes(
    user_id: &str,
    theme_id: i32,
    votes: Vec<Vote>,
    pool: &Pool
) -> Result<(), ServiceError> {
    pool.post_votes(user_id, theme_id, votes).await.map_err(|_| 
        ServiceError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            constants::MESSAGE_INTERNAL_SERVER_ERROR.to_string()
        )
    )
}