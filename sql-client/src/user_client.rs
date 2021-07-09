// Copyright (c) 2019 kenkoooo
// Code released under the MIT license
// https://opensource.org/licenses/mit-license.php

use crate::{PgPool, PgRow, Row, Result};
use crate::models::User;
use async_trait::async_trait;

#[async_trait]
pub trait UserClient {
    async fn signup_user(&self, user: User) -> Result<()>;
    async fn update_user_display_name(&self, user_id: &str, display_name: Option<&str>) -> Result<()>;
    async fn update_user_login_session(&self, user_id: &str, login_session: &str) -> Result<()>;
    async fn get_user_by_id(&self, user_id: &str) -> Result<User>;
}

#[async_trait]
impl UserClient for PgPool {
    async fn signup_user(&self, user: User) -> Result<()> {
        let mut transaction = self.begin().await?;
        sqlx::query(
            r"
            INSERT INTO users (user_id, display_name, hash, login_session)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT DO NOTHING
            ",
        )
        .bind(user.user_id.clone())
        .bind(user.display_name)
        .bind(user.hash)
        .bind(user.login_session)
        .execute(&mut transaction)
        .await?;

        sqlx::query(
            r"
            INSERT INTO profiles (user_id, heart, star, answer, theme, self_vote, top_count)
            VALUES ($1, 0, 0, 0, 0, 0, 0)
            ON CONFLICT DO NOTHING
            ",
        )
        .bind(user.user_id)
        .execute(&mut transaction)
        .await?;

        transaction.commit().await?;

        Ok(())
    }

    async fn update_user_display_name(&self, user_id: &str, display_name: Option<&str>) -> Result<()> {
        sqlx::query(r"UPDATE users SET display_name = $1 WHERE user_id = $2")
        .bind(display_name)
        .bind(user_id)
        .execute(self)
        .await?;
        Ok(())
    }

    async fn update_user_login_session(&self, user_id: &str, login_session: &str) -> Result<()> {
        sqlx::query(r"UPDATE users SET login_session = $1 WHERE user_id = $2")
        .bind(login_session)
        .bind(user_id)
        .execute(self)
        .await?;
        Ok(())
    }

    async fn get_user_by_id(&self, user_id: &str) -> Result<User> {
        let res = sqlx::query(
            r"
            SELECT user_id, display_name, hash, login_session
            FROM users
            WHERE user_id = $1
            ",
        )
        .bind(user_id)
        .try_map(|row: PgRow| {
            let user_id = row.try_get("user_id")?;
            let display_name = row.try_get("display_name")?;
            let hash = row.try_get("hash")?;
            let login_session = row.try_get("login_session")?;
            Ok(User {
                user_id,
                display_name,
                hash,
                login_session
            })
        })
        .fetch_one(self)
        .await?;
        Ok(res)
    }
}
