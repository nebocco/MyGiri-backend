use actix_web::{
    web, HttpResponse, Result,
};
use crate::service::vote_service;
use crate::config::db::Pool;
use crate::models::{
    vote::VoteRequest,
    response::ResponseBody
};
use crate::constants;

// Post /api/theme/{id}/vote
pub async fn post_votes(
    vote_request: web::Json<VoteRequest>,
    pool: web::Data<Pool>
) -> Result<HttpResponse> {
    let VoteRequest{ user_id, theme_id, votes } = vote_request.into_inner();
    match vote_service::post_votes(&user_id, theme_id, votes, pool.get_ref()).await {
        Ok(_) => Ok(
            HttpResponse::Ok()
            .json(ResponseBody::new(constants::MESSAGE_OK, constants::EMPTY))
        ),
        Err(err) => Ok(err.response())
    }
}

// Get /api/theme/{id}/vote
pub async fn summarize_result(
    theme_id: web::Path<i32>,
    pool: web::Data<Pool>
) -> Result<HttpResponse> {
    match vote_service::summarize_result(theme_id.into_inner(), pool.get_ref()).await {
        Ok(themes) => Ok(
            HttpResponse::Ok()
            .set_header("Cache-Control", "max-age=300")
            .json(ResponseBody::new(constants::MESSAGE_OK, themes))),
        Err(err) => Ok(err.response())
    }
}

// Get /api/theme/{id}/vote/{user_id}
pub async fn get_answers_by_user_and_theme(
    theme_id: web::Path<i32>,
    user_id: web::Path<String>,
    pool: web::Data<Pool>
) -> Result<HttpResponse> {
    match vote_service::get_votes_by_user_and_theme(
        user_id.into_inner().as_ref(), theme_id.into_inner(), pool.get_ref()
    ).await {
        Ok(themes) => Ok(
            HttpResponse::Ok()
            .set_header("Cache-Control", "max-age=300")
            .json(ResponseBody::new(constants::MESSAGE_OK, themes))),
        Err(err) => Ok(err.response())
    }
}