// Copyright (c) 2019 kenkoooo
// Code released under the MIT license
// https://opensource.org/licenses/mit-license.php

use crate::{PgPool, PgRow, Row, Result};
use crate::models::Theme;
use async_trait::async_trait;
use chrono::{Date, DateTime, Local, Duration};

#[async_trait]
pub trait ThemeClient {
    async fn get_theme_by_id(&self, theme_id: i32) -> Option<Theme>;
    async fn get_themes_by_user(&self, user_id: &str) -> Result<Vec<Theme>>;
    async fn get_themes_by_date(&self, date: Date<Local>) -> Result<Vec<Theme>>;
    async fn get_themes_to_update(&self, threshold: DateTime<Local>) -> Result<Vec<Theme>>;
    async fn post_theme(&self, theme: Theme) -> Result<i32>;
}

#[async_trait]
impl ThemeClient for PgPool {
    async fn get_theme_by_id(&self, theme_id: i32) -> Option<Theme> {
        let theme = sqlx::query(
            r"
            SELECT 
                t.id,
                t.user_id,
                u.display_name,
                t.epoch_open,
                t.theme_text
            FROM themes AS t
            LEFT JOIN (
                SELECT
                    user_id,
                    display_name
                FROM users
            ) u
            ON t.user_id = u.user_id
            WHERE t.id = $1
            ",
        )
        .bind(theme_id)
        .try_map(|row: PgRow| {
            let id = row.try_get("id")?;
            let user_id = row.try_get("user_id")?;
            let display_name = row.try_get("display_name")?;
            let epoch_open = row.try_get("epoch_open")?;
            let theme_text = row.try_get("theme_text")?;
            Ok(Theme{
                id,
                user_id,
                display_name,
                epoch_open,
                theme_text,
            })
        })
        .fetch_one(self)
        .await
        .ok()?;
        Some(theme)
    }

    async fn get_themes_by_user(&self, user_id: &str) -> Result<Vec<Theme>> {
        let themes = sqlx::query(
            r"
            SELECT 
                t.id,
                t.user_id,
                u.display_name,
                t.epoch_open,
                t.theme_text
            FROM themes AS t
            LEFT JOIN (
                SELECT
                    user_id,
                    display_name
                FROM users
            ) u
            ON t.user_id = u.user_id
            WHERE LOWER(t.user_id) = LOWER($1)
            ",
        )
        .bind(user_id)
        .try_map(|row: PgRow| {
            let id = row.try_get("id")?;
            let user_id = row.try_get("user_id")?;
            let display_name = row.try_get("display_name")?;
            let epoch_open = row.try_get("epoch_open")?;
            let theme_text = row.try_get("theme_text")?;
            Ok(Theme{
                id,
                user_id,
                display_name,
                epoch_open,
                theme_text,
            })
        })
        .fetch_all(self)
        .await?;
        Ok(themes)
    }

    async fn get_themes_by_date(&self, date: Date<Local>) -> Result<Vec<Theme>> {
        let this_day = date.and_hms(0, 0, 0);
        let next_day = this_day + Duration::days(1);
        let themes = sqlx::query(
            r"
            SELECT 
                t.id,
                t.user_id,
                u.display_name,
                t.epoch_open,
                t.theme_text
            FROM themes AS t
            LEFT JOIN (
                SELECT
                    user_id,
                    display_name
                FROM users
            ) u
            ON t.user_id = u.user_id
            WHERE epoch_open >= $1
            AND epoch_open < $2
            ORDER BY epoch_open
            ",
        )
        .bind(this_day)
        .bind(next_day)
        .try_map(|row: PgRow| {
            let id = row.try_get("id")?;
            let user_id = row.try_get("user_id")?;
            let display_name = row.try_get("display_name")?;
            let epoch_open = row.try_get("epoch_open")?;
            let theme_text = row.try_get("theme_text")?;
            Ok(Theme{
                id,
                user_id,
                display_name,
                epoch_open,
                theme_text,
            })
        })
        .fetch_all(self)
        .await?;
        Ok(themes)
    }

    async fn get_themes_to_update(&self, threshold: DateTime<Local>) -> Result<Vec<Theme>> {
        let themes = sqlx::query(
            r"
            SELECT id, user_id, epoch_open, theme_text FROM themes
            WHERE updated = FALSE AND epoch_open < $1
            ",
        )
        .bind(threshold)
        .try_map(|row: PgRow| {
            let id = row.try_get("id")?;
            let user_id = row.try_get("user_id")?;
            let epoch_open = row.try_get("epoch_open")?;
            let theme_text = row.try_get("theme_text")?;
            Ok(Theme{
                id,
                user_id,
                display_name: None,
                epoch_open,
                theme_text,
            })
        })
        .fetch_all(self)
        .await?;
        Ok(themes)
    }


    async fn post_theme(&self, theme: Theme) -> Result<i32> {
        let theme_id: i32 = sqlx::query(
            r"
            INSERT INTO themes (user_id, epoch_open, theme_text)
            VALUES ($1, $2, $3)
            RETURNING id
            ",
        )
        .bind(theme.user_id)
        .bind(theme.epoch_open)
        .bind(theme.theme_text)
        .try_map(|row: PgRow| {
            let id: i32 = row.try_get("id")?;
            Ok(id)
        })
        .fetch_one(self)
        .await?;
        Ok(theme_id)
    }
}
