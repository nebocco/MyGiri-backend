use sql_client::{PgPool, Result};
use sqlx::Executor;
use std::fs::File;
use std::io::prelude::*;

pub type Pool = PgPool;
const SQL_FILE: &str = "./config/init.sql";

pub async fn create_and_initialize_pool(sql_url: &str) -> Result<PgPool> {
    let pool = sql_client::create_pool(sql_url).await?;
    initialize_pool(&pool).await;
    Ok(pool)
}

async fn initialize_pool(pool: &PgPool) {
    let mut file = File::open(SQL_FILE).unwrap();
    let mut sql = String::new();
    file.read_to_string(&mut sql).unwrap();
    let mut conn = pool.acquire().await.unwrap();
    conn.execute(sql.as_str()).await.unwrap();
}