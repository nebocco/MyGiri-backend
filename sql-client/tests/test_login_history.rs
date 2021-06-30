// Copyright (c) 2019 kenkoooo
// Code released under the MIT license
// https://opensource.org/licenses/mit-license.php

use sql_client::models::LoginHistory;
use sql_client::login_history_client::LoginHistoryClient;
use chrono::TimeZone;
mod utils;

#[async_std::test]
async fn test_login_history() {
    let pool = utils::initialize_and_connect_to_test_sql().await;
    
    let histories = vec![
        LoginHistory {
            user_id: "user1".to_string(),
            epoch_login: chrono::Local.ymd(2020, 10, 10).and_hms(17, 30, 00).naive_local()
        },
        LoginHistory {
            user_id: "user2".to_string(),
            epoch_login: chrono::Local.ymd(2020, 10, 10).and_hms(17, 30, 00).naive_local()
        },
        LoginHistory {
            user_id: "user3".to_string(),
            epoch_login: chrono::Local.ymd(2021, 8, 31).and_hms(23, 59, 59).naive_local()
        },
    ];
    for history in histories {
        pool.set_login_history(history).await.unwrap();
    }
    
    let user1 = pool.get_login_history_by_user("user1").await.unwrap();
    assert_eq!(user1.user_id, "user1".to_string());
    assert_eq!(user1.epoch_login, chrono::Local.ymd(2020, 10, 10).and_hms(17, 30, 00).naive_local());

    let user3 = pool.get_login_history_by_user("user3").await.unwrap();
    assert_eq!(user3.user_id, "user3".to_string());
    assert_eq!(user3.epoch_login, chrono::Utc.timestamp(1630454399, 0).naive_local());
    assert_eq!(
        chrono::Local.ymd(2021, 8, 31).and_hms(23, 59, 59).naive_local(),
        chrono::Utc.timestamp(1630454399, 0).naive_local()
    );

    pool.set_login_history(LoginHistory{
        user_id: "user4".to_string(),
        epoch_login: chrono::Local.ymd(2021, 12, 25).and_hms(22, 20, 00).naive_local()
    }).await.unwrap();

    pool.set_login_history(LoginHistory{
        user_id: "user1".to_string(),
        epoch_login: chrono::Local.ymd(2021, 12, 31).and_hms(23, 59, 59).naive_local()
    }).await.unwrap();

    let user4 = pool.get_login_history_by_user("user4").await.unwrap();
    assert_eq!(user4, LoginHistory {
        user_id: "user4".to_string(),
        epoch_login: chrono::Local.ymd(2021, 12, 25).and_hms(22, 20, 00).naive_local()
    });

    let user1 = pool.get_login_history_by_user("user1").await.unwrap();
    assert_eq!(user1, LoginHistory {
        user_id: "user1".to_string(),
        epoch_login: chrono::Local.ymd(2021, 12, 31).and_hms(23, 59, 59).naive_local()
    });

    let err = pool.get_login_history_by_user("do not exist").await;
    assert!(err.is_err());
}