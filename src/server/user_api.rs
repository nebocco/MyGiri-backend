// Copyright (c) 2018 Ba Hai Phan
// Code released under the MIT license
// https://opensource.org/licenses/mit-license.php

use actix_web::{web, HttpRequest, HttpResponse, Result};
use crate::service::user_service;
use crate::config::db::Pool;
use crate::models::{
    user::{UserDTO, UserNameData},
    response::ResponseBody
};
use crate::constants;


// POST api/auth/signup
pub async fn create_user(
    user_dto: web::Json<UserDTO>,
    pool: web::Data<Pool>
) -> Result<HttpResponse> {
    match user_service::create_user(user_dto.0, pool.get_ref()).await {
        Ok(message) => Ok(HttpResponse::Ok().json(ResponseBody::new(&message, constants::EMPTY))),
        Err(err) => Ok(err.response())
    }
}

// POST api/auth/login
pub async fn login(login_dto: web::Json<UserDTO>, pool: web::Data<Pool>) -> Result<HttpResponse> {
    match user_service::login_user(login_dto.0, pool.get_ref()).await {
        Ok(token_res) => Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_LOGIN_SUCCESS, token_res))),
        Err(err) => Ok(err.response()),
    }
}

// POST api/auth/logout
pub async fn logout(req: HttpRequest, pool: web::Data<Pool>) -> Result<HttpResponse> {
    if let Some(authen_header) = req.headers().get(constants::AUTHORIZATION) {
        match user_service::logout_user(authen_header, pool.get_ref()).await {
            Ok(_) => Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_LOGOUT_SUCCESS, constants::EMPTY))),
            Err(err) => Ok(err.response())
        }
    } else {
        Ok(HttpResponse::BadRequest().json(ResponseBody::new(constants::MESSAGE_TOKEN_MISSING, constants::EMPTY)))
    }
}

pub async fn update_name(
    user: web::Json<UserNameData>,
    pool: web::Data<Pool>
) -> Result<HttpResponse> {
    match user_service::update_name(user.0, pool.get_ref()).await {
        Ok(_) => Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_UPDATE_SUCCESS, constants::EMPTY))),
        Err(err) => Ok(err.response())
    }
}