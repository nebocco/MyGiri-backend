// Copyright (c) 2019 kenkoooo
// Code released under the MIT license
// https://opensource.org/licenses/mit-license.php

pub use sqlx::postgres::{PgPool, PgPoolOptions, PgRow};
pub use sqlx::{query, Row};
pub use anyhow::{Error, Result};
use std::time::Duration;

pub mod models;
pub mod user_client;
pub mod login_history_client;
pub mod answer_client;
pub mod theme_client;

pub async fn create_pool<S: AsRef<str>>(database_url: S) -> Result<PgPool> {
    let pool = PgPoolOptions::new()
        .max_lifetime(Duration::from_secs(5 * 60))
        .max_connections(15)
        .connect(database_url.as_ref())
        .await?;
    Ok(pool)
}
