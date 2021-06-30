use sql_client::models::User;
use sql_client::user_client::UserClient;

mod utils;

#[async_std::test]
async fn test_user() {
    let pool = utils::initialize_and_connect_to_test_sql().await;
    
    // direct insert
    sqlx::query(
        r"
        INSERT INTO users (user_id, display_name, hash, login_session) VALUES 
        ('origin1', 'Alice', 'hash1', ''),
        ('origin2', 'Bob', 'hash2', 'ok'),
        ('origin3', null, 'hash3', 'ok?');
        ",
    )
    .execute(&pool)
    .await
    .unwrap();

    // get user
    let origin1 = pool.get_user_by_id("origin1").await.unwrap();
    assert_eq!(origin1.user_id, "origin1".to_string());
    assert_eq!(origin1.display_name, Some("Alice".to_string()));
    assert_eq!(origin1.hash, "hash1".to_string());
    assert_eq!(origin1.login_session, "".to_string());

    let origin3 = pool.get_user_by_id("origin3").await.unwrap();
    assert_eq!(origin3, User {
        user_id: "origin3".to_string(),
        display_name: None,
        hash: "hash3".to_string(),
        login_session: "ok?".to_string(),
    });

    assert!(pool.get_user_by_id("non existing user").await.is_err());

    // signup user
    pool.signup_user(
        User {
            user_id: "origin4".to_string(),
            display_name: Some("David".to_string()),
            hash: "hash4".to_string(),
            login_session: "valid".to_string()
        }
    ).await.unwrap();
    
    pool.signup_user(
        User {
            user_id: "origin2".to_string(),
            display_name: None,
            hash: "invalid registration".to_string(),
            login_session: "invalid".to_string()
        }
    ).await.unwrap();

    let origin4 = pool.get_user_by_id("origin4").await.unwrap();
    assert_eq!(origin4, User {
        user_id: "origin4".to_string(),
        display_name: Some("David".to_string()),
        hash: "hash4".to_string(),
        login_session: "valid".to_string()
    });

    let origin2 = pool.get_user_by_id("origin2").await.unwrap();
    assert_eq!(origin2, User {
        user_id: "origin2".to_string(),
        display_name: Some("Bob".to_string()),
        hash: "hash2".to_string(),
        login_session: "ok".to_string()
    });

    // update user display name
    pool.update_user_display_name(
        "origin1",
        None
    ).await.unwrap();
    let origin1 = pool.get_user_by_id("origin1").await.unwrap();
    assert_eq!(origin1.user_id, "origin1".to_string());
    assert_eq!(origin1.display_name, None);
    assert_eq!(origin1.hash, "hash1".to_string());
    assert_eq!(origin1.login_session, "".to_string());

    pool.update_user_display_name(
        "non existing user",
        Some("I'm Here!")
    ).await.unwrap();
    assert!(pool.get_user_by_id("non existing user").await.is_err());

    // update user login session
    pool.update_user_login_session(
        "origin1",
        "new session"
    ).await.unwrap();
    let origin1 = pool.get_user_by_id("origin1").await.unwrap();
    assert_eq!(origin1.user_id, "origin1".to_string());
    assert_eq!(origin1.display_name, None);
    assert_eq!(origin1.hash, "hash1".to_string());
    assert_eq!(origin1.login_session, "new session".to_string());

    pool.update_user_login_session(
        "non existing user",
        "dummy_session"
    ).await.unwrap();
    assert!(pool.get_user_by_id("non existing user").await.is_err());

    // signup user
    let user_list: Vec<User> = vec![
        ("user1", Some("apple"), "pass1"),
        ("user2", Some("banana"), "pass2"),
        ("user3", None, "pass3"),
    ].into_iter().map(|(user_id, display_name, pwd)| {
        User::new(
            user_id,
            display_name,
            pwd,
        )
    }).collect();
    for user in user_list.into_iter() {
        pool.signup_user(user).await.unwrap();
    }

    // get user
    let user1 = pool.get_user_by_id("user1").await.unwrap();
    assert_eq!(user1.user_id, "user1".to_string());
    assert_eq!(user1.display_name, Some("apple".to_string()));
    assert_eq!(user1.hash, "pass1");

    let user3 = pool.get_user_by_id("user3").await.unwrap();
    assert_eq!(user3.user_id, "user3".to_string());
    assert_eq!(user3.display_name, None);
    assert_eq!(user3.hash, "pass3");
}