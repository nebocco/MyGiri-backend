// Copyright (c) 2019 kenkoooo
// Code released under the MIT license
// https://opensource.org/licenses/mit-license.php

use sql_client::models::{ User, Answer, Theme };
use sql_client::answer_client::AnswerClient;
use sql_client::user_client::UserClient;
use sql_client::theme_client::ThemeClient;
use chrono::TimeZone;
mod utils;

#[async_std::test]
async fn test_answer() {
    let pool = utils::initialize_and_connect_to_test_sql().await;

    let answers = vec![
        Answer {
            id: None,
            user_id: "user1".to_string(),
            display_name: None,
            theme_id: 1,
            epoch_submit: chrono::Local.ymd(2020, 10, 10).and_hms(17, 30, 0),
            answer_text: "answer1".to_string(),
            score: 78,
            voted: false,
        },
        Answer {
            id: Some(8),
            user_id: "USER1".to_string(),
            display_name: None,
            theme_id: 2,
            epoch_submit: chrono::Local.ymd(2020, 10, 10).and_hms(18, 30, 0),
            answer_text: "answer2".to_string(),
            score: 1516,
            voted: false,
        },
        Answer {
            id: None,
            user_id: "user2".to_string(),
            display_name: None,
            theme_id: 1,
            epoch_submit: chrono::Local.ymd(1996, 1, 1).and_hms(7, 3, 0),
            answer_text: "answer3".to_string(),
            score: 271,
            voted: true,
        },
    ];

    // post answer
    for (i, answer) in answers.into_iter().enumerate() {
        let id = pool.post_answer(answer).await.unwrap();
        assert_eq!(id, i as i32 + 1);
    }

    // get answers by user
    let user1 = pool.get_answers_by_user("user1").await.unwrap();
    assert_eq!(user1.len(), 2);
    assert_eq!(user1[0], Answer {
        id: Some(1),
        user_id: "user1".to_string(),
        display_name: None,
        theme_id: 1,
        epoch_submit: chrono::Local.ymd(2020, 10, 10).and_hms(17, 30, 0),
        answer_text: "answer1".to_string(),
        score: 78,
        voted: false,
    });

    assert_eq!(pool.get_answers_by_user("user3").await.unwrap().len(), 0);

    // get answers by theme
    let theme1 = pool.get_answers_by_theme(1).await.unwrap();
    assert_eq!(theme1.len(), 2);
    assert_eq!(theme1[0], Answer {
        id: Some(3),
        user_id: "user2".to_string(),
        display_name: None,
        theme_id: 1,
        epoch_submit: chrono::Local.ymd(1996, 1, 1).and_hms(7, 3, 0),
        answer_text: "answer3".to_string(),
        score: 271,
        voted: true,
    });

    assert_eq!(pool.get_answers_by_theme(0).await.unwrap().len(), 0);
    
    pool.signup_user(User{
        user_id: "user1".to_string(),
        display_name: Some("User 1".to_string()),
        hash: "".to_string(),
        login_session: "".to_string(),
    }).await.unwrap();

    // get answer by user and theme
    let user1_theme1 = pool.get_answer_by_user_and_theme("user1", 1).await.unwrap();
    assert_eq!(user1_theme1, Answer {
        id: Some(1),
        user_id: "user1".to_string(),
        display_name: Some("User 1".to_string()),
        theme_id: 1,
        epoch_submit: chrono::Local.ymd(2020, 10, 10).and_hms(17, 30, 0),
        answer_text: "answer1".to_string(),
        score: 78,
        voted: false,
    });

    let user2_theme2 = pool.get_answer_by_user_and_theme("user2", 2).await;
    assert_eq!(user2_theme2, None);

    // get answers with themes
    let themes = vec![
        Theme {
            id: None,
            user_id: "user1".to_string(),
            display_name: None,
            epoch_open: chrono::Local.ymd(2020, 10, 10).and_hms(18, 30, 0),
            theme_text: "theme1".to_string()
        },
        Theme {
            id: Some(8),
            user_id: "user1".to_string(),
            display_name: None,
            epoch_open: chrono::Local.ymd(2020, 10, 10).and_hms(23, 30, 0),
            theme_text: "theme2".to_string()
        },
        Theme {
            id: None,
            user_id: "user2".to_string(),
            display_name: None,
            epoch_open: chrono::Local.ymd(2021, 1, 13).and_hms(1, 17, 5),
            theme_text: "theme3".to_string()
        },
    ];

    // post theme
    for (i, theme) in themes.into_iter().enumerate() {
        let id = pool.post_theme(theme).await.unwrap();
        assert_eq!((i + 1) as i32, id);
    }

    sqlx::query(
        r"UPDATE themes SET updated = TRUE"
    ).execute(&pool)
    .await
    .unwrap();

    let user1 = pool.get_answers_with_themes_by_user("user1").await.unwrap();
    assert_eq!(user1.len(), 2);
    assert_eq!(user1[0], (
        Answer {
            id: Some(1),
            user_id: "user1".to_string(),
            display_name: Some("User 1".to_string()),
            theme_id: 1,
            epoch_submit: chrono::Local.ymd(2020, 10, 10).and_hms(17, 30, 0),
            answer_text: "answer1".to_string(),
            score: 78,
            voted: false,
        },
        Theme {
            id: Some(1),
            user_id: "user1".to_string(),
            display_name: None,
            epoch_open: chrono::Local.ymd(2020, 10, 10).and_hms(18, 30, 0),
            theme_text: "theme1".to_string()
        }
    ));
}