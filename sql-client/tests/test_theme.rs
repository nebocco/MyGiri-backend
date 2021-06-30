// Copyright (c) 2019 kenkoooo
// Code released under the MIT license
// https://opensource.org/licenses/mit-license.php

use sql_client::models::Theme;
use sql_client::theme_client::ThemeClient;
use chrono::TimeZone;
mod utils;

#[async_std::test]
async fn test_theme() {
    let pool = utils::initialize_and_connect_to_test_sql().await;

    let themes = vec![
        Theme {
            theme_id: None,
            author: "user1".to_string(),
            epoch_open: chrono::Local.ymd(2020, 10, 10).and_hms(18, 30, 0).naive_local(),
            theme_text: "theme1".to_string()
        },
        Theme {
            theme_id: Some(8),
            author: "user1".to_string(),
            epoch_open: chrono::Local.ymd(2020, 10, 10).and_hms(23, 30, 0).naive_local(),
            theme_text: "theme2".to_string()
        },
        Theme {
            theme_id: None,
            author: "user2".to_string(),
            epoch_open: chrono::Local.ymd(2021, 1, 13).and_hms(1, 17, 5).naive_local(),
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
        theme_id: Some(2),
        author: "user1".to_string(),
        epoch_open: chrono::Local.ymd(2020, 10, 10).and_hms(23, 30, 0).naive_local(),
        theme_text: "theme2".to_string()
    });

    let err = pool.get_theme_by_id(4).await;
    assert!(err.is_err());

    // get themes by user
    let user1 = pool.get_themes_by_user("user1").await.unwrap();
    assert_eq!(user1.len(), 2);
    assert_eq!(user1[0], Theme {
        theme_id: Some(1),
        author: "user1".to_string(),
        epoch_open: chrono::Local.ymd(2020, 10, 10).and_hms(18, 30, 0).naive_local(),
        theme_text: "theme1".to_string()
    });

    assert_eq!(pool.get_themes_by_user("user3").await.unwrap().len(), 0);
}