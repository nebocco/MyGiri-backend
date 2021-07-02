// Copyright (c) 2019 kenkoooo
// Code released under the MIT license
// https://opensource.org/licenses/mit-license.php

use crate::{PgPool, PgRow, Row, Result};
use crate::models::{ Answer, Vote };
use async_trait::async_trait;

#[async_trait]
pub trait VoteClient {
    async fn summarize_result(&self, theme_id: i32) -> Result<Vec<Answer>>;
    async fn post_votes(
        &self, 
        user_id: &str,
        theme_id: i32,
        votes: Vec<Vote>
    ) -> Result<()>;
}

#[async_trait]
impl VoteClient for PgPool {
    async fn summarize_result(&self, theme_id: i32) -> Result<Vec<Answer>> {
        let theme = sqlx::query(
            r"
              SELECT 
                a.id,
                user_id,
                theme_id,
                epoch_submit,
                answer_text,
                (CASE WHEN voted = true 100000 ELSE 0 END) AS voted
              FROM answers AS a
              WHERE theme_id = $1
                
              LEFT JOIN (
                SELECT
                  answer_id,
                  SUM(score) as total_score
                FROM votes
                GROUP BY answer_id
              ) v
              ON a.id = v.answer_id

              ORDER BY total_score DESC
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
}
