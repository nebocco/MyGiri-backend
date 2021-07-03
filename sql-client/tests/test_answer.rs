// Copyright (c) 2019 kenkoooo
// Code released under the MIT license
// https://opensource.org/licenses/mit-license.php

use sql_client::models::Answer;
use sql_client::answer_client::AnswerClient;
use chrono::TimeZone;
mod utils;

#[async_std::test]
async fn test_answer() {
    let pool = utils::initialize_and_connect_to_test_sql().await;

    let answers = vec![
        Answer {
            id: None,
            user_id: "user1".to_string(),
            theme_id: 1,
            epoch_submit: chrono::Local.ymd(2020, 10, 10).and_hms(17, 30, 0).naive_local(),
            answer_text: "answer1".to_string(),
            score: 78,
            voted: false,
        },
        Answer {
            id: Some(8),
            user_id: "USER1".to_string(),
            theme_id: 2,
            epoch_submit: chrono::Local.ymd(2020, 10, 10).and_hms(18, 30, 0).naive_local(),
            answer_text: "answer2".to_string(),
            score: 1516,
            voted: false,
        },
        Answer {
            id: None,
            user_id: "user2".to_string(),
            theme_id: 1,
            epoch_submit: chrono::Local.ymd(1996, 1, 1).and_hms(7, 3, 0).naive_local(),
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
        theme_id: 1,
        epoch_submit: chrono::Local.ymd(2020, 10, 10).and_hms(17, 30, 0).naive_local(),
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
        theme_id: 1,
        epoch_submit: chrono::Local.ymd(1996, 1, 1).and_hms(7, 3, 0).naive_local(),
        answer_text: "answer3".to_string(),
        score: 271,
        voted: true,
    });

    assert_eq!(pool.get_answers_by_theme(0).await.unwrap().len(), 0);

    // get answer by user and theme
    let user1_theme1 = pool.get_answer_by_user_and_theme("user1", 1).await.unwrap();
    assert_eq!(user1_theme1, Answer {
        id: Some(1),
        user_id: "user1".to_string(),
        theme_id: 1,
        epoch_submit: chrono::Local.ymd(2020, 10, 10).and_hms(17, 30, 0).naive_local(),
        answer_text: "answer1".to_string(),
        score: 78,
        voted: false,
    });

    let user2_theme2 = pool.get_answer_by_user_and_theme("user2", 2).await;
    assert!(user2_theme2.is_err());

}