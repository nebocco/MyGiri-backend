use actix_web::{
    web, HttpResponse, HttpRequest, Result,
};
use crate::service::answer_service;
use crate::config::db::Pool;
use crate::models::{
    answer::AnswerDTO,
    response::ResponseBody
};
use crate::constants;

// Post /api/theme/{id}/submit
pub async fn post_answer(
    req: HttpRequest,
    answer: web::Json<AnswerDTO>,
    pool: web::Data<Pool>
) -> Result<HttpResponse> {
    let authen_header = req.headers().get(constants::AUTHORIZATION).unwrap();
    match answer_service::post_answer(authen_header, answer.into_inner(), pool.get_ref()).await {
        Ok(_) => Ok(
            HttpResponse::Ok()
            .json(ResponseBody::new(constants::MESSAGE_OK, constants::EMPTY))
        ),
        Err(err) => Ok(err.response())
    }
}

// Get /api/answers/user/{user_id}
pub async fn get_answers_by_user(
    user_id: web::Path<String>,
    pool: web::Data<Pool>
) -> Result<HttpResponse> {
    match answer_service::get_answers_by_user(user_id.into_inner().as_ref(), pool.get_ref()).await {
        Ok(themes) => Ok(
            HttpResponse::Ok()
            .set_header("Cache-Control", "max-age=300")
            .json(ResponseBody::new(constants::MESSAGE_OK, themes))),
        Err(err) => Ok(err.response())
    }
}

// Get /api/answers/theme/{theme_id}
pub async fn get_answers_by_theme(
    theme_id: web::Path<i32>,
    pool: web::Data<Pool>
) -> Result<HttpResponse> {
    match answer_service::get_answers_by_theme(theme_id.into_inner(), pool.get_ref()).await {
        Ok(themes) => Ok(
            HttpResponse::Ok()
            .set_header("Cache-Control", "max-age=300")
            .json(ResponseBody::new(constants::MESSAGE_OK, themes))),
        Err(err) => Ok(err.response())
    }
}

// Get /api/answer/{theme_id}/{user_id}
pub async fn get_answer_by_user_and_theme(
    theme_and_user: web::Path<(i32, String)>,
    pool: web::Data<Pool>
) -> Result<HttpResponse> {
    let (theme_id, user_id) = theme_and_user.into_inner();
    match answer_service::get_answer_by_user_and_theme(user_id.as_ref(), theme_id, pool.get_ref()).await {
        Ok(themes) => Ok(
            HttpResponse::Ok()
            .set_header("Cache-Control", "max-age=300")
            .json(ResponseBody::new(constants::MESSAGE_OK, themes))),
        Err(err) => Ok(err.response())
    }
}