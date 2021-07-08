// Copyright (c) 2018 Ba Hai Phan
// Code released under the MIT license
// https://opensource.org/licenses/mit-license.php

use actix_web::{http, App, HttpServer};
use actix_web::dev::Service;
use actix_cors::Cors;

use sql_client::PgPool;

use crate::config;

pub(crate) mod user_api;
pub(crate) mod theme_api;
pub(crate) mod answer_api;
pub(crate) mod vote_api;
pub(crate) mod profile_api;

pub async fn run_server(pg_pool: PgPool, host: &str, port: u16) -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .wrap(Cors::default() 
            .allowed_origin("http://127.0.0.1:8080")
            .allowed_origin("http://localhost:8080")
            .allowed_origin("https://mygiri.vercel.app")
                .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                .allowed_header(http::header::CONTENT_TYPE)
                .max_age(3600))
            .data(pg_pool.clone())
            .wrap(actix_web::middleware::Logger::default())
            // .wrap(crate::middleware::Authentication)
            .wrap_fn(|req, srv| { srv.call(req) })
            .configure(config::app::config_services)
    })
    .bind((host, port))?
    .run()
    .await
}