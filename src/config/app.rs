// Copyright (c) 2018 Ba Hai Phan
// Code released under the MIT license
// https://opensource.org/licenses/mit-license.php

use crate::server::*;
use actix_web::web;
use log;

pub fn config_services(cfg: &mut web::ServiceConfig) {
    log::info!("Configuring routes...");
    cfg.service(
        web::scope("/api")
            .service(
                web::scope("/auth")
                    .service(
                        web::resource("/signup")
                        .route(web::post().to(user_api::create_user))
                    )
                    .service(
                        web::resource("/login")
                        .route(web::post().to(user_api::login))
                    )
                    .service(
                        web::resource("/logout")
                        .route(web::post().to(user_api::logout))
                    )
            )
            .service(
                web::scope("/user")
                    .service(
                        web::resource("/{user_id}")
                        .route(web::get().to(profile_api::get_profile_by_user))
                        .route(web::post().to(user_api::update_name))
                    )
            )
            .service(
                web::scope("/theme")
                    .service(
                        web::resource("")
                        .route(web::post().to(theme_api::post_theme))
                    )
                    .service(
                        web::scope("/{theme_id}")
                            .service(
                                web::resource("")
                                .route(web::get().to(theme_api::get_theme_by_id))
                                .route(web::post().to(answer_api::post_answer))
                            )
                            .service(
                                web::scope("/vote")
                                    .service(
                                        web::resource("")
                                        .route(web::post().to(vote_api::post_votes))
                                    )
                                    .service(
                                        web::resource("/{user_id}")
                                        .route(web::get().to(vote_api::get_votes_by_user_and_theme))
                                    )
                            )
                            .service(
                                web::resource("/result")
                                .route(web::get().to(vote_api::summarize_result))
                            )
                    )
            )
            .service(
                web::scope("/themes")
                    .service(
                        web::resource("/date/{date}")
                        .route(web::get().to(theme_api::get_themes_by_date))
                    )
                    .service(
                        web::resource("/user/{user_id}")
                        .route(web::get().to(theme_api::get_themes_by_user))
                    )
                    .service(
                        web::resource("recent/{user_id}")
                        .route(web::get().to(theme_api::get_recent_activity))
                    )
                    .service(
                        web::resource("active")
                        .route(web::get().to(theme_api::get_themes_active))
                    )
            )
            .service(
                web::scope("/answer")
                    .service(
                        web::resource("/{theme_id}/{user_id}")
                        .route(web::get().to(answer_api::get_answer_by_user_and_theme))
                    )
            )
            .service(
                web::scope("/answers")
                    .service(
                        web::resource("/theme/{theme_id}")
                        .route(web::get().to(answer_api::get_answers_by_theme))
                    )
                    .service(
                        web::resource("/user/{user_id}")
                        .route(web::get().to(answer_api::get_answers_by_user))
                    )
            )
    );
}