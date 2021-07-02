use actix_web::{
    web, HttpResponse, Result,
};
use crate::service::theme_service;
use crate::config::db::Pool;
use crate::models::{
    response::ResponseBody
};
use crate::constants;
use chrono::NaiveDate;

// Get /api/theme/{id}
pub async fn get_theme_by_id(
    id: web::Path<i32>,
    pool: web::Data<Pool>
) -> Result<HttpResponse> {
    match theme_service::get_theme_by_id(id.into_inner(), pool.get_ref()).await {
        Ok(theme) => Ok(
            HttpResponse::Ok()
            .set_header("Cache-Control", "max-age=300")
            .json(ResponseBody::new(constants::MESSAGE_OK, theme))
        ),
        Err(err) => Ok(err.response())
    }
}

// Get /api/themes/date/{date}
pub async fn get_themes_by_date(
    date: web::Path<String>,
    pool: web::Data<Pool>
) -> Result<HttpResponse> {
    let date = NaiveDate::parse_from_str(date.into_inner().as_ref(), "%Y-%m-%d").unwrap();
    match theme_service::get_themes_by_date(date, pool.get_ref()).await {
        Ok(themes) => Ok(
            HttpResponse::Ok()
            .set_header("Cache-Control", "max-age=300")
            .json(ResponseBody::new(constants::MESSAGE_OK, themes))),
        Err(err) => Ok(err.response())
    }
}

// Get /api/themes/user/{user_id}
pub async fn get_themes_by_user(
    user_id: web::Path<String>,
    pool: web::Data<Pool>
) -> Result<HttpResponse> {
    match theme_service::get_themes_by_user(user_id.into_inner().as_ref(), pool.get_ref()).await {
        Ok(themes) => Ok(
            HttpResponse::Ok()
            .set_header("Cache-Control", "max-age=300")
            .json(ResponseBody::new(constants::MESSAGE_OK, themes))),
        Err(err) => Ok(err.response())
    }
}