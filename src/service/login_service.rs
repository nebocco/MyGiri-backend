use uuid::Uuid;
use sql_client::{
    user_client::UserClient,
    login_history_client::LoginHistoryClient,
    models::LoginHistory,
};
use crate::{
    config::db::Pool,
    constants,
    errors::{ ServiceError, StatusCode }
};


pub async fn create_login_history(user_id: &str, pool: &Pool) -> Result<LoginHistory, ServiceError> {
    let user = pool.get_user_by_id(user_id).await
    .map_err(|_| ServiceError::new(
        StatusCode::UNAUTHORIZED,
        constants::MESSAGE_USER_NOT_FOUND.to_string()
    ))?;
    let now = chrono::Local::now();
    Ok(LoginHistory{
        user_id: user.user_id,
        epoch_login: now.naive_local()
    })
}

pub async fn save_login_history(lh: LoginHistory, pool: &Pool) -> Result<(), ServiceError> {
    pool.set_login_history(lh).await.map_err(|_|
        ServiceError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            constants::MESSAGE_INTERNAL_SERVER_ERROR.to_string()
        )
    )
}

pub fn generate_login_session() -> String {
    Uuid::new_v4().to_simple().to_string()
}