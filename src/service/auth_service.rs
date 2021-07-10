// Copyright (c) 2018 Ba Hai Phan
// Code released under the MIT license
// https://opensource.org/licenses/mit-license.php

use crate::{
    models::{
        user::{ User, UserDTO, UserNameData },
        login::{ LoginDTO, UserToken, TokenBodyResponse }
    },
    config::db::Pool,
    utils::*,
    constants,
    errors::{ ServiceError, StatusCode }
};
use sql_client::{
    user_client::UserClient,
    login_history_client::LoginHistoryClient,
    models::{ LoginHistory }
};
use actix_web::http::header::HeaderValue;

use uuid::Uuid;

pub async fn create_user(user: UserDTO, pool: &Pool) -> Result<String, ServiceError> {
    if user.user_id.len() == 0 {
        return Err(
            ServiceError::new(
                StatusCode::BAD_REQUEST,
                "user_id is empty".to_string()
            )
        )
    }

    if user.user_id.len() > 30 {
        return Err(
            ServiceError::new(
                StatusCode::BAD_REQUEST,
                "user_id is too long".to_string()
            )
        )
    }

    if user.password.len() < 8 {
        return Err(
            ServiceError::new(
                StatusCode::BAD_REQUEST,
                "password is too short".to_string()
            )
        )
    }

    if user.password.len() > 30 {
        return Err(
            ServiceError::new(
                StatusCode::BAD_REQUEST,
                "password is too long".to_string()
            )
        )
    }

    if user.display_name.as_deref().unwrap_or("this is ok").len() == 0 {
        return Err(
            ServiceError::new(
                StatusCode::BAD_REQUEST,
                "display_name is empty".to_string()
            )
        )
    }

    if user.display_name.as_deref().unwrap_or("this is ok").len() > 60 {
        return Err(
            ServiceError::new(
                StatusCode::BAD_REQUEST,
                "display_name is too long".to_string()
            )
        )
    }
    
    if pool.get_user_by_id(&user.user_id).await.is_some() {
        return Err(ServiceError::new(
            StatusCode::BAD_REQUEST,
            format!("User '{}' is already registered", &user.user_id)
        ));
    }
    let hashed_pwd = hash_password(&user.password).unwrap();
    let new_user = User::new(&user.user_id, user.display_name.as_deref(), &hashed_pwd);
    if pool.signup_user(new_user).await.is_ok() {
        Ok(constants::MESSAGE_SIGNUP_SUCCESS.to_string())
    } else {
        Err(ServiceError::new(
            StatusCode::BAD_REQUEST,
            constants::MESSAGE_SIGNUP_FAILED.to_string()
        ))
    }
}

pub async fn login_user(login: UserDTO, pool: &Pool) -> Result<TokenBodyResponse, ServiceError> {
    let user = pool.get_user_by_id(&login.user_id).await
        .ok_or(
            ServiceError::new(
                StatusCode::UNAUTHORIZED,
                constants::MESSAGE_USER_NOT_FOUND.to_string()
            )
        )?;

    if !verify_password(&user.hash, &login.password)
        .map_err(|_| ServiceError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            constants::MESSAGE_INTERNAL_SERVER_ERROR.to_string()
        ))?
    {
        return Err(ServiceError::new(
            StatusCode::UNAUTHORIZED,
            constants::MESSAGE_LOGIN_FAILED.to_string()
        ));
    }
    let login_history = create_login_history(&user.user_id, pool).await?;
    save_login_history(login_history, pool).await?;
    let login_session = generate_login_session();
    pool.update_user_login_session(&user.user_id, &login_session).await
        .map_err(|_| ServiceError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            constants::MESSAGE_INTERNAL_SERVER_ERROR.to_string()
        ))?;

    let login_dto = LoginDTO {
        user,
        login_session: login_session,
    };

    // ??
    let token_res = TokenBodyResponse::new(&login_dto);
    Ok(token_res)
}

pub async fn logout_user(authen_header: &HeaderValue, pool: &Pool) -> Result<(), ServiceError> {
    let error = |_| ServiceError::new(
        StatusCode::INTERNAL_SERVER_ERROR,
        constants::MESSAGE_PROCESS_TOKEN_ERROR.to_string()
    );
    let authen_str = authen_header.to_str().map_err(|_| ()).map_err(error)?;
    if !authen_str.starts_with("bearer") { return Err(error(()));}
    let token = authen_str[6..authen_str.len()].trim();
    let token_data = decode_token(token.to_string()).map_err(|_| ()).map_err(error)?;
    let user_id = verify_token(&token_data, pool).await.map_err(|_| ()).map_err(error)?;
    pool.update_user_login_session(&user_id, "").await
        .map_err(|_| ServiceError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            constants::MESSAGE_DB_CONNECTION_ERROR.to_string()
        ))?;
    Ok(())
}

pub async fn is_valid_login_session(user_token: &UserToken, pool: &Pool) -> Result<bool, String> {
    let user = pool.get_user_by_id(&user_token.user_id).await
        .ok_or(constants::MESSAGE_DB_CONNECTION_ERROR.to_string())?;
    Ok(user.login_session == user_token.login_session)
}

pub async fn update_name(authen_header: &HeaderValue, user:UserNameData, pool: &Pool) -> Result<(), ServiceError> {
    let error = |_| ServiceError::new(
        StatusCode::INTERNAL_SERVER_ERROR,
        constants::MESSAGE_PROCESS_TOKEN_ERROR.to_string()
    );
    let authen_str = authen_header.to_str().map_err(|_| ()).map_err(error)?;
    if !authen_str.starts_with("bearer") { return Err(error(()));}
    let token = authen_str[6..authen_str.len()].trim();
    let token_data = decode_token(token.to_string()).map_err(|_| ()).map_err(error)?.claims;
    if token_data.user_id != user.user_id {
        return Err(
            ServiceError::new(
                StatusCode::BAD_REQUEST,
                constants::MESSAGE_INVALID_TOKEN.to_string()
            )
        )
    }

    if user.display_name.as_deref().unwrap_or("this is ok").len() == 0 {
        return Err(
            ServiceError::new(
                StatusCode::BAD_REQUEST,
                "display_name is empty".to_string()
            )
        )
    }

    if user.display_name.as_deref().unwrap_or("this is ok").len() > 60 {
        return Err(
            ServiceError::new(
                StatusCode::BAD_REQUEST,
                "display_name is too long".to_string()
            )
        )
    }
    
    pool.update_user_display_name(&user.user_id, user.display_name.as_deref()).await
        .map_err(|_| ServiceError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            constants::MESSAGE_INTERNAL_SERVER_ERROR.to_string()
        ))?;

    Ok(())
}

pub async fn create_login_history(user_id: &str, pool: &Pool) -> Result<LoginHistory, ServiceError> {
    let user = pool.get_user_by_id(user_id).await
    .ok_or(ServiceError::new(
        StatusCode::UNAUTHORIZED,
        constants::MESSAGE_USER_NOT_FOUND.to_string()
    ))?;
    Ok(LoginHistory{
        user_id: user.user_id,
        epoch_login: chrono::Local::now()
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