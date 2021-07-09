use actix_web::{
    web, HttpResponse, HttpRequest, Result,
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
    req: HttpRequest,
    vote_request: web::Json<VoteRequest>,
    pool: web::Data<Pool>
) -> Result<HttpResponse> {
    let authen_header = req.headers().get(constants::AUTHORIZATION).unwrap();
    let VoteRequest{ user_id, theme_id, votes } = vote_request.into_inner();
    match vote_service::post_votes(authen_header, &user_id, theme_id, votes, pool.get_ref()).await {
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
pub async fn get_votes_by_user_and_theme(
    params: web::Path<(i32, String)>,
    pool: web::Data<Pool>
) -> Result<HttpResponse> {
    let (theme_id, user_id) = params.into_inner();
    match vote_service::get_votes_by_user_and_theme(
        user_id.as_ref(), theme_id, pool.get_ref()
    ).await {
        Ok(themes) => Ok(
            HttpResponse::Ok()
            .set_header("Cache-Control", "max-age=300")
            .json(ResponseBody::new(constants::MESSAGE_OK, themes))),
        Err(err) => Ok(err.response())
    }
}