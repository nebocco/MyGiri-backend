mod utils;
use utils::*;
 

#[actix_rt::test]
async fn test_signup_ok() {
    let pool = initialize_and_connect_to_test_sql().await;

    let mut app = test::init_service(
        App::new()
        .wrap(Cors::default()
            .send_wildcard()
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_header(header::CONTENT_TYPE)
            .max_age(3600))
        .data(pool)
        .wrap(Logger::default())
        // .wrap(crate::middleware::auth_middleware::Authentication)
        // .wrap_fn(|req, srv| srv.call(req).map(|res| res))
        .configure(config::app::config_services)
    ).await;

    let resp = test::TestRequest::post()
        .uri("/api/auth/signup")
        .set(header::ContentType::json())
        .set_payload(r#"{"user_id":"admin","password":"pass"}"#.as_bytes())
        .send_request(&mut app)
        .await;

    // let data = test::read_body(resp).await;

    // println!("{:#?}", &data);
    assert_eq!(resp.status(), StatusCode::OK);
}

#[actix_rt::test]
async fn test_signup_duplicate_user() {
    let pool = initialize_and_connect_to_test_sql().await;

    let mut app = test::init_service(
        App::new()
        .wrap(Cors::default()
            .send_wildcard()
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_header(header::CONTENT_TYPE)
            .max_age(3600))
        .data(pool)
        .wrap(Logger::default())
        // .wrap(crate::middleware::auth_middleware::Authentication)
        // .wrap_fn(|req, srv| srv.call(req).map(|res| res))
        .configure(config::app::config_services)
    ).await;

    test::TestRequest::post()
        .uri("/api/auth/signup")
        .set(header::ContentType::json())
        .set_payload(r#"{"user_id":"admin","password":"pass"}"#.as_bytes())
        .send_request(&mut app)
        .await;

    let resp = test::TestRequest::post()
        .uri("/api/auth/signup")
        .set(header::ContentType::json())
        .set_payload(r#"{"user_id":"admin","password":"pass"}"#.as_bytes())
        .send_request(&mut app)
        .await;

    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}

#[actix_rt::test]
async fn test_login() {
    let pool = initialize_and_connect_to_test_sql().await;

    let mut app = test::init_service(
        App::new()
        .wrap(Cors::default()
            .send_wildcard()
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_header(header::CONTENT_TYPE)
            .max_age(3600))
        .data(pool)
        .wrap(Logger::default())
        // .wrap(crate::middleware::auth_middleware::Authentication)
        // .wrap_fn(|req, srv| srv.call(req).map(|res| res))
        .configure(config::app::config_services)
    ).await;

    test::TestRequest::post()
        .uri("/api/auth/signup")
        .set(header::ContentType::json())
        .set_payload(r#"{"user_id":"admin","password":"pass"}"#.as_bytes())
        .send_request(&mut app)
        .await;

    let resp = test::TestRequest::post()
        .uri("/api/auth/login")
        .set(header::ContentType::json())
        .set_payload(r#"{"user_id":"admin","password":"pass"}"#.as_bytes())
        .send_request(&mut app)
        .await;
    
    // let data = test::read_body(resp).await;
    // println!("{:#?}", &data); assert!(false);

    assert_eq!(resp.status(), StatusCode::OK);


    // incorrect password
    let resp = test::TestRequest::post()
        .uri("/api/auth/login")
        .set(header::ContentType::json())
        .set_payload(r#"{"user_id":"admin","password":"password"}"#.as_bytes())
        .send_request(&mut app)
        .await;

    // let data = test::read_body(resp).await;
    // println!("{:#?}", &data); assert!(false);

    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);


    // user not found
    let resp = test::TestRequest::post()
        .uri("/api/auth/login")
        .set(header::ContentType::json())
        .set_payload(r#"{"user_id":"this user doesn't exist","password":"password"}"#.as_bytes())
        .send_request(&mut app)
        .await;

    // let data = test::read_body(resp).await;
    // println!("{:#?}", &data); assert!(false);

    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
}