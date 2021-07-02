// Copyright (c) 2018 Ba Hai Phan
// Code released under the MIT license
// https://opensource.org/licenses/mit-license.php

use std::env;
use actix_backend::server::run_server;
use actix_backend::config::db;

#[actix_web::main]
async fn main() {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=debug");
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    let database_url = env::var("SQL_URL").expect("SQL_URL is not set.");
    let port: u16 = 8008;

    let pg_pool = db::create_and_initialize_pool(&database_url)
        .await
        .expect("Failed to initialize the connection pool");

    run_server(pg_pool, port).await.expect("Failed to run server");
}