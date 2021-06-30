use crate::{
    models::{
        user::{UserDTO, UserNameData},
        login::{ LoginDTO, UserToken, TokenBodyResponse }
    },
    config::db::Pool,
    utils::*,
    constants,
    service::login_service::{
        create_login_history,
        save_login_history, 
        generate_login_session
    },
    errors::{ ServiceError, StatusCode }
};
use sql_client::user_client::UserClient;
use sql_client::models::User;
use actix_web::http::header::HeaderValue;

pub async fn create_user(user: UserDTO, pool: &Pool) -> Result<String, ServiceError> {
    if pool.get_user_by_id(&user.user_id).await.is_ok() {
        return Err(ServiceError::new(
            StatusCode::BAD_REQUEST,
            format!("User '{}' is already registered", &user.user_id)
        ));
    }
    let hashed_pwd = hash_password(&user.password).unwrap();
    let new_user = User::new(&user.user_id, None, &hashed_pwd);
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
        .map_err(|_|
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
        .map_err(|_| constants::MESSAGE_DB_CONNECTION_ERROR.to_string())?;
    Ok(user.login_session == user_token.login_session)
}

pub async fn update_name(user:UserNameData, pool: &Pool) -> Result<(), ServiceError> {
    pool.update_user_display_name(&user.user_id, user.display_name.as_deref()).await
        .map_err(|_| ServiceError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            constants::MESSAGE_INTERNAL_SERVER_ERROR.to_string()
        ))?;

    Ok(())
}