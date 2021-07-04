// Copyright (c) 2018 Ba Hai Phan
// Code released under the MIT license
// https://opensource.org/licenses/mit-license.php

use crate::{
    models::{
        answer::{ Answer, AnswerDTO },
    },
    constants,
    config::db::Pool,
    errors::{ ServiceError, StatusCode }
};
use sql_client::answer_client::AnswerClient;

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

pub async fn post_answer(answer_dto: AnswerDTO, pool: &Pool) -> Result<i32, ServiceError> {
    let answer = Answer {
        id: None,
        user_id: answer_dto.user_id,
        theme_id: answer_dto.theme_id,
        epoch_submit: chrono::Local::now().naive_local(),
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