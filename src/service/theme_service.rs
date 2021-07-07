use crate::{
    models::{
        theme::{ Theme, ThemeDTO }
    },
    constants,
    config::db::Pool,
    errors::{ ServiceError, StatusCode }
};
use sql_client::theme_client::ThemeClient;
use chrono::{TimeZone, Date, Local};

pub async fn get_theme_by_id(theme_id: i32, pool: &Pool) -> Result<Theme, ServiceError> {
    pool.get_theme_by_id(theme_id).await.map_err(|_| 
        ServiceError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            constants::MESSAGE_INTERNAL_SERVER_ERROR.to_string()
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

pub async fn post_theme(theme_dto: ThemeDTO, pool: &Pool) -> Result<i32, ServiceError> {
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