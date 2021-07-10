// Copyright (c) 2018 Ba Hai Phan
// Code released under the MIT license
// https://opensource.org/licenses/mit-license.php

use crate::{
    models::{
        answer::{ Answer, AnswerDTO },
    },
    utils::*,
    constants,
    config::db::Pool,
    errors::{ ServiceError, StatusCode }
};
use sql_client::answer_client::AnswerClient;
use actix_web::http::header::HeaderValue;

pub async fn get_answers_by_theme(theme_id: i32, pool: &Pool) -> Result<Vec<Answer>, ServiceError> {
    log::info!("{:?}", pool.get_answers_by_theme(theme_id).await);
    
    pool.get_answers_by_theme(theme_id).await.map_err(|_| 
        ServiceError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            constants::MESSAGE_INTERNAL_SERVER_ERROR.to_string()
        )
    )
}

pub async fn get_answers_by_user(user_id: &str, pool: &Pool) -> Result<Vec<Answer>, ServiceError> {
    pool.get_answers_by_user(user_id).await.map_err(|_| 
        ServiceError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            constants::MESSAGE_INTERNAL_SERVER_ERROR.to_string()
        )
    )
}

pub async fn get_answer_by_user_and_theme(user_id: &str, theme_id: i32, pool: &Pool) -> Result<Answer, ServiceError> {
    match pool.get_answer_by_user_and_theme(user_id, theme_id).await {
        Some(res) => Ok(res),
        None => Err(
            ServiceError::new(
                StatusCode::NOT_FOUND,
                constants::EMPTY.to_string()
            )
        )
    }
}

pub async fn post_answer(authen_header: &HeaderValue, answer_dto: AnswerDTO, pool: &Pool) -> Result<i32, ServiceError> {
    let error = |_| ServiceError::new(
        StatusCode::INTERNAL_SERVER_ERROR,
        constants::MESSAGE_PROCESS_TOKEN_ERROR.to_string()
    );
    let authen_str = authen_header.to_str().map_err(|_| ()).map_err(error)?;
    if !authen_str.starts_with("bearer") { return Err(error(()));}
    let token = authen_str[6..authen_str.len()].trim();
    let token_data = decode_token(token.to_string()).map_err(|_| ()).map_err(error)?.claims;
    if token_data.user_id != answer_dto.user_id {
        return Err(
            ServiceError::new(
                StatusCode::BAD_REQUEST,
                constants::MESSAGE_INVALID_TOKEN.to_string()
            )
        )
    }

    if answer_dto.answer_text.len() == 0 {
        return Err(
            ServiceError::new(
                StatusCode::BAD_REQUEST,
                "answer_text is empty".to_string()
            )
        )
    }

    if answer_dto.answer_text.len() > 250 {
        return Err(
            ServiceError::new(
                StatusCode::BAD_REQUEST,
                "answer_text is too long".to_string()
            )
        )
    }

    let answer = Answer {
        id: None,
        user_id: answer_dto.user_id,
        display_name: None,
        theme_id: answer_dto.theme_id,
        epoch_submit: chrono::Local::now(),
        answer_text: answer_dto.answer_text,
        score: 0,
        voted: false
    };
    
    pool.post_answer(answer).await.map_err(|_| 
        ServiceError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            constants::MESSAGE_INTERNAL_SERVER_ERROR.to_string()
        )
    )
}