use sql_client::PgPool;
use sqlx::Executor;
use std::fs::File;
use std::io::prelude::*;

const SQL_FILE: &str = "../config/init.sql";
const SQL_URL_ENV_KEY: &str = "SQL_URL";

pub async fn initialize_and_connect_to_test_sql() -> PgPool {
    let sql_url = std::env::var(SQL_URL_ENV_KEY).unwrap();
    let pool = sql_client::create_pool(sql_url).await.unwrap();
    initialize_pool(&pool).await;
    pool
}

async fn initialize_pool(pool: &PgPool) {
    let mut file = File::open(SQL_FILE).unwrap();
    let mut sql = String::new();
    file.read_to_string(&mut sql).unwrap();
    let mut conn = pool.acquire().await.unwrap();
    conn.execute(sql.as_str()).await.unwrap();
}
