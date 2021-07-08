use actix_web::{
    web, HttpResponse, Result,
};
use crate::service::profile_service;
use crate::config::db::Pool;
use crate::constants;
use crate::models::response::ResponseBody;

// Get /api/user/{user_id}
pub async fn get_profile_by_user(
    user_id: web::Path<String>,
    pool: web::Data<Pool>
) -> Result<HttpResponse> {
    match profile_service::get_profile_by_user(user_id.into_inner().as_ref(), pool.get_ref()).await {
        Ok(profile) => Ok(
            HttpResponse::Ok()
            .json(ResponseBody::new(constants::MESSAGE_OK, profile))
        ),
        Err(err) => Ok(err.response())
    }
}