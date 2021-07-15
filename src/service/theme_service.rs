use crate::{
    models::{
        theme::{ Theme, ThemeDTO }
    },
    utils::*,
    constants,
    config::db::Pool,
    errors::{ ServiceError, StatusCode }
};
use sql_client::theme_client::ThemeClient;
use actix_web::http::header::HeaderValue;
use chrono::{TimeZone, Date, Local};

pub async fn get_theme_by_id(theme_id: i32, pool: &Pool) -> Result<Theme, ServiceError> {
    pool.get_theme_by_id(theme_id).await.ok_or(
        ServiceError::new(
            StatusCode::NOT_FOUND,
            constants::EMPTY.to_string()
        )
    )
}

pub async fn get_themes_by_date(date: Date<Local>, pool: &Pool) -> Result<Vec<Theme>, ServiceError> {
    pool.get_themes_by_date(date).await.map_err(|_|
        ServiceError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            constants::MESSAGE_INTERNAL_SERVER_ERROR.to_string()
        )
    )
}

pub async fn get_themes_by_user(user_id: &str, pool: &Pool) -> Result<Vec<Theme>, ServiceError> {
    pool.get_themes_by_user(user_id).await.map_err(|_|
        ServiceError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            constants::MESSAGE_INTERNAL_SERVER_ERROR.to_string()
        )
    )
}

pub async fn get_recent_activity(user_id: &str, pool: &Pool) -> Result<Vec<Theme>, ServiceError> {
    pool.get_recent_activity(user_id).await.map_err(|_|
        ServiceError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            constants::MESSAGE_INTERNAL_SERVER_ERROR.to_string()
        )
    )
}

pub async fn get_themes_active(pool: &Pool) -> Result<Vec<Theme>, ServiceError> {
    let now = chrono::Local::now();
    pool.get_themes_active(now).await.map_err(|_|
        ServiceError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            constants::MESSAGE_INTERNAL_SERVER_ERROR.to_string()
        )
    )
}


pub async fn post_theme(authen_header: &HeaderValue, theme_dto: ThemeDTO, pool: &Pool) -> Result<i32, ServiceError> {
    let error = |_| ServiceError::new(
        StatusCode::INTERNAL_SERVER_ERROR,
        constants::MESSAGE_PROCESS_TOKEN_ERROR.to_string()
    );
    let authen_str = authen_header.to_str().map_err(|_| ()).map_err(error)?;
    if !authen_str.starts_with("bearer") { return Err(error(()));}
    let token = authen_str[6..authen_str.len()].trim();
    let token_data = decode_token(token.to_string()).map_err(|_| ()).map_err(error)?.claims;
    if token_data.user_id != theme_dto.user_id {
        return Err(
            ServiceError::new(
                StatusCode::BAD_REQUEST,
                constants::MESSAGE_INVALID_TOKEN.to_string()
            )
        )
    }

    if theme_dto.theme_text.len() == 0 {
        return Err(
            ServiceError::new(
                StatusCode::BAD_REQUEST,
                "theme_text is empty".to_string()
            )
        )
    }

    if theme_dto.theme_text.chars().count() > 100 {
        return Err(
            ServiceError::new(
                StatusCode::BAD_REQUEST,
                "theme_text is too long".to_string()
            )
        )
    }

    let theme = Theme {
        id: None,
        user_id: theme_dto.user_id,
        display_name: None,
        theme_text: theme_dto.theme_text,
        epoch_open: chrono::Local.ymd(2999, 12, 31).and_hms(0, 0, 0)
    };

    pool.post_theme(theme).await.map_err(|_|
        ServiceError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            constants::MESSAGE_INTERNAL_SERVER_ERROR.to_string()
        )
    )
}