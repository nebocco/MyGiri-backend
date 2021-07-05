// Copyright (c) 2018 Ba Hai Phan
// Code released under the MIT license
// https://opensource.org/licenses/mit-license.php

use std::env;
use actix_backend::server::run_server;
use sql_client::create_pool;

#[actix_web::main]
async fn main() {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set.");
    let host: String = std::env::var("HOST").expect("HOST is not set.");
    let port: u16 = std::env::var("PORT_API").expect("PORT_API is not set.")
        .parse::<u16>().unwrap();

    let pg_pool = create_pool(&database_url)
        .await
        .expect("Failed to initialize the connection pool");

    run_server(pg_pool, host.as_ref(), port).await.expect("Failed to run server");
}