// Copyright (c) 2019 kenkoooo
// Code released under the MIT license
// https://opensource.org/licenses/mit-license.php

use sql_client::models::{ Theme, User, Answer };
use sql_client::theme_client::ThemeClient;
use sql_client::user_client::UserClient;
use sql_client::answer_client::AnswerClient;
use chrono::TimeZone;
mod utils;

#[async_std::test]
async fn test_theme() {
    let pool = utils::initialize_and_connect_to_test_sql().await;

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

    // get theme by id
    let theme2 = pool.get_theme_by_id(2).await.unwrap();
    assert_eq!(theme2, Theme {
        id: Some(2),
        user_id: "user1".to_string(),
        display_name: None,
        epoch_open: chrono::Local.ymd(2020, 10, 10).and_hms(23, 30, 0),
        theme_text: "theme2".to_string()
    });

    let err = pool.get_theme_by_id(4).await;
    assert!(err.is_none());

    // get themes by user
    let user1 = pool.get_themes_by_user("user1").await.unwrap();
    assert_eq!(user1.len(), 2);
    assert_eq!(user1[0], Theme {
        id: Some(1),
        user_id: "user1".to_string(),
        display_name: None,
        epoch_open: chrono::Local.ymd(2020, 10, 10).and_hms(18, 30, 0),
        theme_text: "theme1".to_string()
    });

    pool.signup_user(User{
        user_id: "user1".to_string(),
        display_name: Some("User 1".to_string()),
        hash: "".to_string(),
        login_session: "".to_string(),
    }).await.unwrap();

    assert_eq!(pool.get_themes_by_user("user3").await.unwrap().len(), 0);

    // get themes of a day
    let today = chrono::Local.ymd(2020, 10, 10);
    let themes = pool.get_themes_by_date(today).await.unwrap();
    assert_eq!(themes.len(), 2);
    assert_eq!(themes[0], Theme {
        id: Some(1),
        user_id: "user1".to_string(),
        display_name: Some("User 1".to_string()),
        epoch_open: chrono::Local.ymd(2020, 10, 10).and_hms(18, 30, 0),
        theme_text: "theme1".to_string()
    });

    let threshold = chrono::Local.ymd(2020, 10, 10).and_hms(22, 0, 0);
    let themes = pool.get_themes_to_update(threshold).await.unwrap();
    assert_eq!(themes.len(), 1);
    assert_eq!(themes[0], Theme {
        id: Some(1),
        user_id: "user1".to_string(),
        display_name: None,
        epoch_open: chrono::Local.ymd(2020, 10, 10).and_hms(18, 30, 0),
        theme_text: "theme1".to_string()
    });

    sqlx::query(
        r"
        UPDATE themes
        SET updated = TRUE
        WHERE id = 1
        "
    ).execute(&pool)
    .await.unwrap();

    // get themes active
    let active = pool.get_themes_active().await.unwrap();
    assert_eq!(active.len(), 2);
    assert_eq!(active[0], Theme {
        id: Some(2),
        user_id: "user1".to_string(),
        display_name: Some("User 1".to_string()),
        epoch_open: chrono::Local.ymd(2020, 10, 10).and_hms(23, 30, 0),
        theme_text: "theme2".to_string()
    });


    // Get recent activity
    pool.post_answer(Answer::new("user3", 2, "user3_answer")).await.unwrap();
    pool.post_answer(Answer::new("user3", 1, "user3_answer")).await.unwrap();
    pool.post_answer(Answer::new("user4", 1, "user4_answer")).await.unwrap();
    pool.post_answer(Answer::new("user1", 2, "user1_answer")).await.unwrap();

    let user3 = pool.get_recent_activity("user3").await.unwrap();
    assert_eq!(user3.len(), 2);
    assert_eq!(user3[0], Theme {
        id: Some(1),
        user_id: "user1".to_string(),
        display_name: Some("User 1".to_string()),
        epoch_open: chrono::Local.ymd(2020, 10, 10).and_hms(18, 30, 0),
        theme_text: "theme1".to_string()
    });
}