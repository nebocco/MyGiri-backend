// Copyright (c) 2019 kenkoooo
// Code released under the MIT license
// https://opensource.org/licenses/mit-license.php

use sql_client::models::{ Vote, Answer, VoteResult, User };
use sql_client::vote_client::VoteClient;
use sql_client::answer_client::AnswerClient;
use sql_client::user_client::UserClient;
use chrono::TimeZone;
mod utils;

#[async_std::test]
async fn test_vote() {
    let pool = utils::initialize_and_connect_to_test_sql().await;

    let init_votes: Vec<Vote> = vec![
        ("user1", 1, 1, 1),
        ("user1", 1, 2, 10),
        ("user1", 2, 4, 100),
        ("user2", 2, 4, 1000),
        ("user2", 2, 5, 10000),
        ("user3", 1, 1, 100000),
        ("user3", 1, 2, 1000000),
        ("user3", 2, 4, 10000000),
        ("user3", 2, 5, 100000000),
        ("user3", 2, 6, 1000000000),
        ].into_iter()
        .map(|(user_id, theme_id, answer_id, score)| 
        Vote {
            user_id: user_id.to_string(),
            theme_id,
            answer_id,
            score
        }
    ).collect();

    pool.post_votes("dummy", 0, init_votes).await.unwrap();

    // get votes by user and theme
    let user1_theme1 = pool.get_votes_by_user_and_theme("user1", 1).await.unwrap();
    assert_eq!(user1_theme1.len(), 2);
    assert_eq!(user1_theme1[0], Vote {
        user_id: "user1".to_string(),
        theme_id: 1,
        answer_id: 1,
        score: 1
    });

    // summerize result
    let answers = vec![
        Answer {
            id: None,
            user_id: "user1".to_string(),
            theme_id: 1,
            epoch_submit: chrono::Local.ymd(2020, 10, 10).and_hms(17, 30, 0).naive_local(),
            answer_text: "answer1".to_string(),
            score: 0,
            voted: false,
        },
        Answer {
            id: Some(8),
            user_id: "user2".to_string(),
            theme_id: 1,
            epoch_submit: chrono::Local.ymd(2020, 10, 10).and_hms(18, 30, 0).naive_local(),
            answer_text: "answer2".to_string(),
            score: 0,
            voted: false,
        },
        Answer {
            id: None,
            user_id: "user3".to_string(),
            theme_id: 1,
            epoch_submit: chrono::Local.ymd(1996, 1, 1).and_hms(7, 3, 0).naive_local(),
            answer_text: "answer3".to_string(),
            score: 0,
            voted: true,
        },
    ];
    for answer in answers {
        pool.post_answer(answer).await.unwrap();
    }
    let theme1 = pool.summarize_result(1).await.unwrap();
    assert_eq!(theme1.len(), 3);
    assert_eq!(theme1[0], VoteResult {
        id: Some(2),
        user_id: "user2".to_string(),
        display_name: None,
        theme_id: 1,
        epoch_submit: chrono::Local.ymd(2020, 10, 10).and_hms(18, 30, 0).naive_local(),
        answer_text: "answer2".to_string(),
        score: 1000010,
        voted: false,
    });
    assert_eq!(theme1[2], VoteResult {
        id: Some(3),
        user_id: "user3".to_string(),
        display_name: None,
        theme_id: 1,
        epoch_submit: chrono::Local.ymd(1996, 1, 1).and_hms(7, 3, 0).naive_local(),
        answer_text: "answer3".to_string(),
        score: 100000,
        voted: true,
    });

    let user1: Vec<Vote> = vec![
        ("user1", 1, 3, 1000000000),
        ("user1", 2, 6, 100),
        ].into_iter()
        .map(|(user_id, theme_id, answer_id, score)| 
        Vote {
            user_id: user_id.to_string(),
            theme_id,
            answer_id,
            score
        }
    ).collect();

    pool.post_votes("user1", 1, user1).await.unwrap();
    pool.signup_user(User {
        user_id: "user3".to_string(),
        display_name: Some("ゆーざー".to_string()),
        hash: "".to_string(),
        login_session: "".to_string()
    }).await.unwrap();
    let theme1 = pool.summarize_result(1).await.unwrap();
    println!("{:?}", &theme1);
    assert_eq!(theme1.len(), 3);
    assert_eq!(theme1[0], VoteResult {
        id: Some(3),
        user_id: "user3".to_string(),
        display_name: Some("ゆーざー".to_string()),
        theme_id: 1,
        epoch_submit: chrono::Local.ymd(1996, 1, 1).and_hms(7, 3, 0).naive_local(),
        answer_text: "answer3".to_string(),
        score: 1000100000,
        voted: true,
    });
    assert_eq!(theme1[1], VoteResult {
        id: Some(2),
        user_id: "user2".to_string(),
        display_name: None,
        theme_id: 1,
        epoch_submit: chrono::Local.ymd(2020, 10, 10).and_hms(18, 30, 0).naive_local(),
        answer_text: "answer2".to_string(),
        score: 1000000,
        voted: false,
    });
    assert_eq!(theme1[2], VoteResult {
        id: Some(1),
        user_id: "user1".to_string(),
        display_name: None,
        theme_id: 1,
        epoch_submit: chrono::Local.ymd(2020, 10, 10).and_hms(17, 30, 0).naive_local(),
        answer_text: "answer1".to_string(),
        score: 200000,
        voted: true,
    })
}