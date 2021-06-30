pub(crate) use actix_backend::config::{self, db::Pool};
pub(crate) use actix_web::{
    App, 
    test, 
    http::{StatusCode, header},
    middleware::Logger
};
pub(crate) use actix_cors::Cors;
pub(crate) use sqlx::Executor;
pub(crate) use std::fs::File;
pub(crate) use std::io::prelude::*;

const SQL_FILE: &str = "./config/init.sql";
const SQL_URL_ENV_KEY: &str = "SQL_URL";

pub async fn initialize_and_connect_to_test_sql() -> Pool {
    let sql_url = std::env::var(SQL_URL_ENV_KEY).unwrap();
    let pool = sql_client::create_pool(sql_url).await.unwrap();
    initialize_pool(&pool).await;
    pool
}

async fn initialize_pool(pool: &Pool) {
    let mut file = File::open(SQL_FILE).unwrap();
    let mut sql = String::new();
    file.read_to_string(&mut sql).unwrap();
    let mut conn = pool.acquire().await.unwrap();
    conn.execute(sql.as_str()).await.unwrap();
}