use crate::{
    models::{
        theme::Theme
    },
    constants,
    config::db::Pool,
    errors::{ ServiceError, StatusCode }
};
use sql_client::theme_client::ThemeClient;
use chrono::NaiveDate;

pub async fn get_theme_by_id(theme_id: i32, pool: &Pool) -> Result<Theme, ServiceError> {
    pool.get_theme_by_id(theme_id).await.map_err(|_| 
        ServiceError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            constants::MESSAGE_INTERNAL_SERVER_ERROR.to_string()
        )
    )
}

pub async fn get_themes_by_date(date: NaiveDate, pool: &Pool) -> Result<Vec<Theme>, ServiceError> {
    pool.get_themes_of_a_day(date).await.map_err(|_| 
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