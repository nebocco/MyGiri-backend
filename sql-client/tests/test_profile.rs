// Copyright (c) 2019 kenkoooo
// Code released under the MIT license
// https://opensource.org/licenses/mit-license.php

use sql_client::models::Profile;
use sql_client::profile_client::ProfileClient;
use sql_client::theme_client::ThemeClient;
mod utils;
use chrono::TimeZone;

#[async_std::test]
async fn test_profile() {
    let pool = utils::initialize_and_connect_to_test_sql().await;

    sqlx::query(
        r"
        INSERT INTO users (user_id, display_name, hash, login_session) VALUES
        ('user1', 'USER 1', '', ''),
        ('user2', 'USER 2', '', ''),
        ('user3', 'USER 3', '', '')
        "
    )
    .execute(&pool).await.unwrap();

    sqlx::query(
        r"
        INSERT INTO profiles (user_id, heart, star, answer, theme, self_vote, top_count) VALUES
        ('user1', 0, 0, 0, 0, 0, 0),
        ('user2', 10, 10, 10, 10, 10, 10),
        ('user3', 100, 100, 100, 100, 100, 100)
        "
    )
    .execute(&pool).await.unwrap();

    let user2 = pool.get_profile_by_user("user2").await.unwrap();
    assert_eq!(user2, Profile {
        user_id: "user2".to_string(),
        display_name: Some("USER 2".to_string()),
        heart: 10,
        star: 10,
        answer: 10,
        theme: 10,
        self_vote: 10,
        top_count: 10
    });

    sqlx::query(
        r"
        INSERT INTO answers(user_id, theme_id, epoch_submit, answer_text, score, voted) VALUES
        ('user2', 1, '2021-07-06 15:01:34.138663+09:00', 'user2_answer', 0, FALSE),
        ('user3', 1, '2021-07-06 15:01:34.138663+09:00', 'user3_answer', 0, TRUE)
        "
    )
    .execute(&pool).await.unwrap();

    sqlx::query(
        r"
        INSERT INTO themes (user_id, epoch_open, theme_text) VALUES
        ('user1', '2021-07-06 15:01:34.138663+09:00', 'user1_theme')
        "
    )
    .execute(&pool).await.unwrap();

    sqlx::query(
        r"
        INSERT INTO votes (user_id, theme_id, answer_id, score) VALUES
        ('user1', 1, 1, 1),
        ('user1', 1, 2, 100000),
        ('user4', 1, 1, 100000),
        ('user3', 1, 2, 1)
        "
    )
    .execute(&pool).await.unwrap();

    pool.update_profile(1).await.unwrap();
    let user1 = pool.get_profile_by_user("user1").await.unwrap();
    assert_eq!(user1, Profile {
        user_id: "user1".to_string(),
        display_name: Some("USER 1".to_string()),
        heart: 0,
        star: 0,
        answer: 0,
        theme: 1,
        self_vote: 0,
        top_count: 0
    });

    let user2 = pool.get_profile_by_user("user2").await.unwrap();
    assert_eq!(user2, Profile {
        user_id: "user2".to_string(),
        display_name: Some("USER 2".to_string()),
        heart: 11,
        star: 11,
        answer: 11,
        theme: 10,
        self_vote: 10,
        top_count: 10
    });

    let user3 = pool.get_profile_by_user("user3").await.unwrap();
    assert_eq!(user3, Profile {
        user_id: "user3".to_string(),
        display_name: Some("USER 3".to_string()),
        heart: 102,
        star: 101,
        answer: 101,
        theme: 100,
        self_vote: 101,
        top_count: 101
    });

    let themes = pool.get_themes_to_update(chrono::Local.ymd(2021, 7, 10).and_hms(10, 12, 0)).await.unwrap();
    assert_eq!(themes, Vec::new());
}