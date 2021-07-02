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
                        web::resource("/name")
                            .route(web::post().to(user_api::update_name))
                    )
            )
            .service(
                web::scope("/theme")
                    .service(
                        web::resource("/{id}")
                            .route(web::get().to(theme_api::get_theme_by_id))
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
            )
            // .service(
            //     web::scope("/address-book")
            //         .service(
            //             web::resource("")
            //                 .route(web::get().to(address_book_controller::find_all))
            //                 .route(web::post().to(address_book_controller::insert))
            //         )
            //         .service(
            //             web::resource("/{id}")
            //                 .route(web::get().to(address_book_controller::find_by_id))
            //                 .route(web::put().to(address_book_controller::update))
            //                 .route(web::delete().to(address_book_controller::delete))
            //         )
            //         .service(
            //             web::resource("/query/{query}")
            //                 .route(web::get().to(address_book_controller::query))   
            //         )
            // )
    );
}