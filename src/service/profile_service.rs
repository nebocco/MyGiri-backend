// Copyright (c) 2018 Ba Hai Phan
// Code released under the MIT license
// https://opensource.org/licenses/mit-license.php

use crate::{
    models::profile::Profile,
    constants,
    config::db::Pool,
    errors::{ ServiceError, StatusCode }
};
use sql_client::profile_client::ProfileClient;

pub async fn get_profile_by_user(user_id: &str, pool: &Pool) -> Result<Profile, ServiceError> {
    pool.get_profile_by_user(user_id).await.ok_or( 
        ServiceError::new(
            StatusCode::NOT_FOUND,
            constants::EMPTY.to_string()
        )
    )
}