// Copyright (c) 2019 kenkoooo
// Code released under the MIT license
// https://opensource.org/licenses/mit-license.php

use crate::{PgPool, PgRow, Row, Result};
use crate::models::{ Theme, Profile };
use crate::vote_client::VoteClient;
use async_trait::async_trait;

#[async_trait]
pub trait ProfileClient {
    async fn get_profile_by_user(&self, user_id: &str) -> Option<Profile>;
    async fn update_profile(&self, theme: Theme) -> Result<()>;
}

#[async_trait]
impl ProfileClient for PgPool {
    async fn get_profile_by_user(&self, user_id: &str) -> Option<Profile> {
        let profile = sqlx::query(
            r"
            SELECT
                p.user_id,
                u.display_name,
                p.heart,
                p.star,
                p.answer,
                p.theme,
                p.self_vote,
                p.top_count
            FROM profiles AS p
            LEFT JOIN (
                SELECT
                    user_id,
                    display_name
                FROM users
            ) u
            ON LOWER(p.user_id) = LOWER(u.user_id)
            WHERE p.user_id = $1
            "
        )
        .bind(user_id)
        .try_map(|row: PgRow| {
            let user_id = row.try_get("user_id")?;
            let display_name = row.try_get("display_name")?;
            let heart = row.try_get("heart")?;
            let star = row.try_get("star")?;
            let answer = row.try_get("answer")?;
            let theme = row.try_get("theme")?;
            let self_vote = row.try_get("self_vote")?;
            let top_count = row.try_get("top_count")?;
            
            Ok(Profile {
                user_id,
                display_name,
                heart,
                star,
                answer,
                theme,
                self_vote,
                top_count
            })
        })
        .fetch_one(self)
        .await
        .ok()?;

        Some(profile)
    }
    
    async fn update_profile(&self, theme: Theme) -> Result<()> {
        let answers = self.summarize_result(theme.id.unwrap()).await?;
        let (user_ids, hearts, stars, answers, themes, self_votes, top_counts) =
        answers.into_iter().enumerate()
        .fold(
            (Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new()),
            |(
                mut user_ids,
                mut hearts,
                mut stars,
                mut answers,
                mut themes,
                mut self_votes,
                mut top_counts
            ), (i, answer)| {
                user_ids.push(answer.user_id.clone());
                hearts.push(answer.score / 100000);
                stars.push(answer.score % 100000);
                answers.push(1);
                themes.push(0);
                self_votes.push(if answer.voted { 1 } else { 0 });
                top_counts.push(if i == 0 { 1 } else { 0 });
                (user_ids, hearts, stars, answers, themes, self_votes, top_counts)
            }
        );

        let mut transaction = self.begin().await?;
        sqlx::query(
            r"
            INSERT INTO profiles AS p (user_id, heart, star, answer,
                theme, self_vote, top_count) VALUES
            (
                UNNEST($1::VARCHAR(100)[]),
				UNNEST($2::INTEGER[]),
				UNNEST($3::INTEGER[]),
				UNNEST($4::INTEGER[]),
				UNNEST($5::INTEGER[]),
				UNNEST($6::INTEGER[]),
				UNNEST($7::INTEGER[])
            )
            ON CONFLICT (user_id) DO UPDATE SET
            heart = p.heart + EXCLUDED.heart,
            star = p.star + EXCLUDED.star,
            answer = p.answer + EXCLUDED.answer,
            theme = p.theme + EXCLUDED.theme,
            self_vote = p.self_vote + EXCLUDED.self_vote,
            top_count = p.top_count + EXCLUDED.top_count
            "
        )
        .bind(user_ids)
        .bind(hearts)
        .bind(stars)
        .bind(answers)
        .bind(themes)
        .bind(self_votes)
        .bind(top_counts)
        .execute(&mut transaction)
        .await?;

        sqlx::query(
            r"
            UPDATE profiles SET theme = theme + 1 WHERE user_id = $1
            "
        )
        .bind(theme.user_id)
        .execute(&mut transaction)
        .await?;

        sqlx::query(
            r"
            UPDATE themes SET updated = TRUE WHERE id = $1
            "
        )
        .bind(theme.id)
        .execute(&mut transaction)
        .await?;

        transaction.commit().await?;

        Ok(())
    }
}
