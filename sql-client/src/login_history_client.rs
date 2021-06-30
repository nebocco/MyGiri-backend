// Copyright (c) 2019 kenkoooo
// Code released under the MIT license
// https://opensource.org/licenses/mit-license.php

use crate::{PgPool, Row, PgRow, Result};
use crate::models::LoginHistory;
use async_trait::async_trait;

#[async_trait]
pub trait LoginHistoryClient {
    async fn set_login_history(&self, lh: LoginHistory) -> Result<()>;
    async fn get_login_history_by_user(&self, user_id: &str) -> Result<LoginHistory>;
}

#[async_trait]
impl LoginHistoryClient for PgPool {
    async fn set_login_history(&self, lh: LoginHistory) -> Result<()>{
        sqlx::query(
            r"
            INSERT INTO login_history (user_id, epoch_login)
            VALUES ($1, $2)
            ON CONFLICT (user_id) DO UPDATE
            SET epoch_login = EXCLUDED.epoch_login
            ",
        )
        .bind(lh.user_id)
        .bind(lh.epoch_login)
        .execute(self)
        .await?;
        Ok(())
    }

    async fn get_login_history_by_user(&self, user_id: &str) -> Result<LoginHistory>{
        let login_history = sqlx::query(
            r"
            SELECT user_id, epoch_login FROM login_history
            WHERE LOWER(user_id) = $1
            ",
        )
        .bind(user_id)
        .try_map(|row: PgRow| {
            let user_id = row.try_get("user_id")?;
            let epoch_login = row.try_get("epoch_login")?;
            Ok(LoginHistory {
                user_id,
                epoch_login
            })
        })
        .fetch_one(self)
        .await?;
        Ok(login_history)
    }
}