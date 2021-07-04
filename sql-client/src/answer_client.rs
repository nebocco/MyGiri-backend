// Copyright (c) 2019 kenkoooo
// Code released under the MIT license
// https://opensource.org/licenses/mit-license.php

use crate::{PgPool, PgRow, Row, Result};
use crate::models::Answer;
use async_trait::async_trait;

#[async_trait]
pub trait AnswerClient {
    async fn get_answers_by_user(&self, user_id: &str) -> Result<Vec<Answer>>;
    async fn get_answers_by_theme(&self, theme_id: i32) -> Result<Vec<Answer>>;
    async fn get_answer_by_user_and_theme(&self, user_id: &str, theme_id: i32) -> Result<Answer>;
    async fn post_answer(&self, answer: Answer) -> Result<i32>;
}

#[async_trait]
impl AnswerClient for PgPool {
    async fn get_answers_by_user(&self, user_id: &str) -> Result<Vec<Answer>> {
        let answers = sqlx::query(
            r"
            SELECT id, user_id, theme_id, epoch_submit, answer_text, score, voted
            FROM answers
            WHERE LOWER(user_id) = LOWER($1)
            ORDER BY epoch_submit ASC
            ",
        )
        .bind(user_id)
        .try_map(|row: PgRow| {
            let id = row.try_get("id")?;
            let user_id = row.try_get("user_id")?;
            let theme_id = row.try_get("theme_id")?;
            let epoch_submit = row.try_get("epoch_submit")?;
            let answer_text = row.try_get("answer_text")?;
            let score: i32 = row.try_get("score")?;
            let voted = row.try_get("voted")?;
            Ok(Answer{
                id: Some(id),
                user_id,
                theme_id,
                epoch_submit,
                answer_text,
                score: score as i64,
                voted
            })
        })
        .fetch_all(self)
        .await?;
        Ok(answers)
    }

    async fn get_answers_by_theme(&self, theme_id: i32) -> Result<Vec<Answer>> {
        let answers = sqlx::query(
            r"
            SELECT id, user_id, theme_id, epoch_submit, answer_text, score, voted
            FROM answers
            WHERE theme_id = $1
            ORDER BY epoch_submit ASC
            ",
        )
        .bind(theme_id)
        .try_map(|row: PgRow| {
            let id = row.try_get("id")?;
            let user_id = row.try_get("user_id")?;
            let theme_id = row.try_get("theme_id")?;
            let epoch_submit = row.try_get("epoch_submit")?;
            let answer_text = row.try_get("answer_text")?;
            let score: i32 = row.try_get("score")?;
            let voted = row.try_get("voted")?;
            Ok(Answer{
                id: Some(id),
                user_id,
                theme_id,
                epoch_submit,
                answer_text,
                score: score as i64,
                voted
            })
        })
        .fetch_all(self)
        .await?;
        Ok(answers)
    }

    async fn get_answer_by_user_and_theme(&self, user_id: &str, theme_id: i32) -> Result<Answer> {
        let answer = sqlx::query(
            r"
            SELECT id, user_id, theme_id, epoch_submit, answer_text, score, voted
            FROM answers
            WHERE LOWER(user_id) = LOWER($1)
            AND theme_id = $2
            ",
        )
        .bind(user_id)
        .bind(theme_id)
        .try_map(|row: PgRow| {
            let id = row.try_get("id")?;
            let user_id = row.try_get("user_id")?;
            let theme_id = row.try_get("theme_id")?;
            let epoch_submit = row.try_get("epoch_submit")?;
            let answer_text = row.try_get("answer_text")?;
            let score: i32 = row.try_get("score")?;
            let voted = row.try_get("voted")?;
            Ok(Answer{
                id: Some(id),
                user_id,
                theme_id,
                epoch_submit,
                answer_text,
                score: score as i64,
                voted
            })
        })
        .fetch_one(self)
        .await?;
        Ok(answer)
    }

    async fn post_answer(&self, answer: Answer) -> Result<i32> {
        let id = sqlx::query(
            r"
            INSERT INTO answers (user_id, theme_id, epoch_submit, answer_text, score, voted)
            VALUES ($1, $2, $3, $4, $5, $6)
            ON CONFLICT (user_id, theme_id) DO UPDATE SET
            epoch_submit = EXCLUDED.epoch_submit,
            answer_text = EXCLUDED.answer_text,
            score = EXCLUDED.score
            RETURNING id
            ",
        )
        .bind(answer.user_id)
        .bind(answer.theme_id)
        .bind(answer.epoch_submit)
        .bind(answer.answer_text)
        .bind(answer.score)
        .bind(answer.voted)
        .try_map(|row: PgRow| {
            let id = row.try_get("id")?;
            Ok(id)
        })
        .fetch_one(self)
        .await?;
        Ok(id)
    }
}
