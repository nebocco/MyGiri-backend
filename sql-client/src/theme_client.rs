// Copyright (c) 2019 kenkoooo
// Code released under the MIT license
// https://opensource.org/licenses/mit-license.php

use crate::{PgPool, PgRow, Row, Result};
use crate::models::Theme;
use async_trait::async_trait;

#[async_trait]
pub trait ThemeClient {
    async fn get_theme_by_id(&self, theme_id: i32) -> Result<Theme>;
    async fn get_themes_by_user(&self, user_id: &str) -> Result<Vec<Theme>>;
    async fn post_theme(&self, theme: Theme) -> Result<i32>;
}

#[async_trait]
impl ThemeClient for PgPool {
    async fn get_theme_by_id(&self, theme_id: i32) -> Result<Theme> {
        let theme = sqlx::query(
            r"
            SELECT theme_id, author, epoch_open, theme_text FROM themes
            WHERE theme_id = $1
            ",
        )
        .bind(theme_id)
        .try_map(|row: PgRow| {
            let theme_id = row.try_get("theme_id")?;
            let author = row.try_get("author")?;
            let epoch_open = row.try_get("epoch_open")?;
            let theme_text = row.try_get("theme_text")?;
            Ok(Theme{
                theme_id: Some(theme_id),
                author,
                epoch_open,
                theme_text,
            })
        })
        .fetch_one(self)
        .await?;
        Ok(theme)
    }

    async fn get_themes_by_user(&self, user_id: &str) -> Result<Vec<Theme>> {
        let themes = sqlx::query(
            r"
            SELECT theme_id, author, epoch_open, theme_text FROM themes
            WHERE LOWER(author) = LOWER($1)
            ",
        )
        .bind(user_id)
        .try_map(|row: PgRow| {
            let theme_id = row.try_get("theme_id")?;
            let author = row.try_get("author")?;
            let epoch_open = row.try_get("epoch_open")?;
            let theme_text = row.try_get("theme_text")?;
            Ok(Theme{
                theme_id: Some(theme_id),
                author,
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
            INSERT INTO themes (author, epoch_open, theme_text)
            VALUES ($1, $2, $3)
            RETURNING theme, _id
            ",
        )
        .bind(theme.author)
        .bind(theme.epoch_open)
        .bind(theme.theme_text)
        .try_map(|row: PgRow| {
            let theme_id: i32 = row.try_get("theme_id")?;
            Ok(theme_id)
        })
        .fetch_one(self)
        .await?;
        Ok(theme_id)
    }
}
