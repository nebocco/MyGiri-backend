use actix_web::{
    web, HttpResponse, HttpRequest, Result,
};
use crate::service::theme_service;
use crate::config::db::Pool;
use crate::models::{
    response::ResponseBody,
    theme::ThemeDTO
};
use crate::constants;
use chrono::{ Date, TimeZone, Local, NaiveDate };

// Get /api/theme/{id}
pub async fn get_theme_by_id(
    theme_id: web::Path<i32>,
    pool: web::Data<Pool>
) -> Result<HttpResponse> {
    match theme_service::get_theme_by_id(theme_id.into_inner(), pool.get_ref()).await {
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
    let naive_date = NaiveDate::parse_from_str(date.into_inner().as_ref(), "%Y-%m-%d").unwrap();
    let date: Date<Local> = Local.from_local_date(&naive_date).unwrap();
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

pub async fn post_theme(
    req: HttpRequest,
    theme_dto: web::Json<ThemeDTO>,
    pool: web::Data<Pool>
) -> Result<HttpResponse> {
    let authen_header = req.headers().get(constants::AUTHORIZATION).unwrap();
    match theme_service::post_theme(authen_header, theme_dto.into_inner(), pool.get_ref()).await {
        Ok(_) => Ok(
            HttpResponse::Ok()
            .json(ResponseBody::new(constants::MESSAGE_OK, constants::EMPTY))
        ),
        Err(err) => Ok(err.response())
    }
}