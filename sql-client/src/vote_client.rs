// Copyright (c) 2019 kenkoooo
// Code released under the MIT license
// https://opensource.org/licenses/mit-license.php

use crate::{PgPool, PgRow, Row, Result};
use crate::models::{ Answer, Vote };
use async_trait::async_trait;

#[async_trait]
pub trait VoteClient {
    async fn get_votes_by_user_and_theme(&self, user_id: &str, theme_id: i32) -> Result<Vec<Vote>>;
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
    async fn get_votes_by_user_and_theme(&self, user_id: &str, theme_id: i32) -> Result<Vec<Vote>> {
        let votes = sqlx::query(
            r"
            SELECT user_id, theme_id, answer_id, score FROM votes
            WHERE user_id = $1 AND theme_id = $2
            "
        )
        .bind(user_id)
        .bind(theme_id)
        .try_map(|row: PgRow| {
            let user_id = row.try_get("user_id")?;
            let theme_id = row.try_get("theme_id")?;
            let answer_id = row.try_get("answer_id")?;
            let score = row.try_get("score")?;
            Ok(Vote{
                user_id,
                theme_id,
                answer_id,
                score
            })
        })
        .fetch_all(self)
        .await?;

        Ok(votes)
    }
    async fn summarize_result(&self, theme_id: i32) -> Result<Vec<Answer>> {
        let mut answers = sqlx::query(
            r"
            SELECT 
                a.id,
                a.user_id,
                u.display_name,
                a.theme_id,
                a.epoch_submit,
                a.answer_text,
                a.voted,
                COALESCE(v.score, 0) AS score
            FROM answers AS a                
            LEFT JOIN (
                SELECT
                answer_id,
                SUM(score) AS score
                FROM votes
                GROUP BY answer_id
            ) v
            ON a.id = v.answer_id
            LEFT JOIN (
                SELECT
                user_id,
                display_name
                FROM users
            ) u
            ON a.user_id = u.user_id
            WHERE theme_id = $1
            ",
        )
        .bind(theme_id)
        .try_map(|row: PgRow| {
            let id = row.try_get("id")?;
            let user_id = row.try_get("user_id")?;
            let display_name = row.try_get("display_name")?;
            let theme_id = row.try_get("theme_id")?;
            let epoch_submit = row.try_get("epoch_submit")?;
            let answer_text = row.try_get("answer_text")?;
            let score: i64 = row.try_get("score")?;
            let voted = row.try_get("voted")?;
            Ok(Answer {
                id: Some(id),
                user_id,
                display_name,
                theme_id,
                epoch_submit,
                answer_text,
                score: score + if voted { 100_000 } else { 0 },
                voted
            })
        })
        .fetch_all(self)
        .await?;
        answers.sort_by_key(|a| (-a.score, a.voted, a.epoch_submit));
        Ok(answers)
    }

    async fn post_votes(&self, user_id: &str, theme_id: i32, votes: Vec<Vote>) -> Result<()> {
        let (user_ids, theme_ids, answer_ids, scores) = votes
		.into_iter().fold(
			(Vec::new(), Vec::new(), Vec::new(), Vec::new()),
			|(mut uids, mut tids, mut aids, mut scs), vote| {
				uids.push(vote.user_id);
				tids.push(vote.theme_id);
				aids.push(vote.answer_id);
				scs.push(vote.score);
				(uids, tids, aids, scs)
			}
		);
		
        let mut transaction = self.begin().await?;

        sqlx::query(
            r"
            DELETE FROM votes WHERE user_id = $1 AND theme_id = $2
            "
        )
        .bind(user_id)
        .bind(theme_id)
        .execute(&mut transaction)
        .await?;

		sqlx::query(
            r"
			INSERT INTO votes (user_id, theme_id, answer_id, score)
			VALUES (
				UNNEST($1::VARCHAR(100)[]),
				UNNEST($2::INTEGER[]),
				UNNEST($3::INTEGER[]),
				UNNEST($4::INTEGER[])
			)
			"
        )
        .bind(user_ids)
        .bind(theme_ids)
        .bind(answer_ids)
        .bind(scores)
        .execute(&mut transaction)
        .await?;

        sqlx::query(
            r"
            UPDATE answers SET voted = true
            WHERE user_id = $1 AND theme_id = $2
            "
        )
        .bind(user_id)
        .bind(theme_id)
        .execute(&mut transaction)
        .await?;

        transaction.commit().await?;
        Ok(())
    }
}
