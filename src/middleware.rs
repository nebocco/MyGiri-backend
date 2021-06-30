use crate::{config::db::Pool, constants, models::response::ResponseBody, utils};
use actix_web::{
    Error, HttpResponse,
    dev::{Service, Transform, ServiceRequest, ServiceResponse},
    http::{Method, HeaderName, HeaderValue},
    web::Data,
};
use async_std::task;
use std::{
    future::{ready, Ready, Future},
    pin::Pin,
    task::{Context, Poll},
};

pub struct Authentication;

impl<S, B> Transform<S> for Authentication
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthenticationMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthenticationMiddleware { service }))
    }
}
pub struct AuthenticationMiddleware<S> {
    service: S,
}

impl<S, B> Service for AuthenticationMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, mut req: ServiceRequest) -> Self::Future {
        let headers = req.headers_mut();
        headers.append(HeaderName::from_static("content-length"), HeaderValue::from_static("true"));

        let mut authenticate_pass = Method::OPTIONS == *req.method() ||
        constants::IGNORE_ROUTES.iter().any(|ignore_route|
            req.path().starts_with(ignore_route)
        );
        if !authenticate_pass {
            log::info!("Need Authentication");
            if let Some(pool) = req.app_data::<Data<Pool>>() {
                log::info!("Connecting to database...");
                if let Some(authen_header) = req.headers().get(constants::AUTHORIZATION) {
                    log::info!("Parsing authorization header...");
                    if let Ok(authen_str) = authen_header.to_str() {
                        if authen_str.starts_with("bearer") || authen_str.starts_with("Bearer") {
                            log::info!("Parsing token...");
                            let token = authen_str[6..authen_str.len()].trim();
                            if let Ok(token_data) = utils::decode_token(token.to_string()) {
                                log::info!("Decoding token...");
                                if task::block_on(utils::verify_token(&token_data, pool)).is_ok() {
                                    log::info!("Valid token");
                                    authenticate_pass = true;
                                } else {
                                    log::error!("Invalid token");
                                }
                            }
                        }
                    }
                }
            }
        }
        if authenticate_pass {
            let fut = self.service.call(req);
            Box::pin(async move {
                let res = fut.await?;
                Ok(res)
            })
        } else {
            Box::pin(async move {
                Ok(req.into_response(
                HttpResponse::Unauthorized()
                    .json(ResponseBody::new(
                        constants::MESSAGE_INVALID_TOKEN,
                        constants::EMPTY,
                    ))
                    .into_body(),
                ))
            })
        }
    }
}