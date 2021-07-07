// Copyright (c) 2019 kenkoooo
// Code released under the MIT license
// https://opensource.org/licenses/mit-license.php

use serde::{Serialize, Deserialize};
use chrono::{Local, DateTime};

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct User {
    pub user_id: String,
    pub display_name: Option<String>,
    pub hash: String,
    pub login_session: String
}

impl User {
    pub fn new(
        user_id: &str,
        display_name: Option<&str>,
        hash: &str,
    ) -> Self {
        User {
            user_id: user_id.to_string(),
            display_name: display_name.map(|v| v.to_string()),
            hash: hash.to_string(),
            login_session: "".to_string()
        }
    }
}


#[derive(Debug, PartialEq, Serialize)]
pub struct Theme {
    pub id: Option<i32>,
    pub user_id: String,
    pub display_name: Option<String>,
    pub epoch_open: DateTime<Local>,
    pub theme_text: String
}

impl Theme {
    pub fn new(
        user_id: &str,
        epoch_open: impl Into<DateTime<Local>>,
        theme_text: &str
    ) -> Self {
        Theme {
            id: None,
            user_id: user_id.to_string(),
            display_name: None,
            epoch_open: epoch_open.into(),
            theme_text: theme_text.to_string()
        }
    }
}

#[derive(Debug, PartialEq, Serialize)]
pub struct Answer {
    pub id: Option<i32>,
    pub user_id: String,
    pub display_name: Option<String>,
    pub theme_id: i32,
    pub epoch_submit: DateTime<Local>,
    pub answer_text: String,
    pub score: i64,
    pub voted: bool
}

impl Answer {
    pub fn new(
        user_id: &str,
        theme_id: i32,
        text: &str
    ) -> Self {
        Answer {
            id: None,
            user_id: user_id.to_string(),
            display_name: None,
            theme_id,
            epoch_submit: chrono::Local::now(),
            answer_text: text.to_string(),
            score: 0,
            voted: false
        }
    }
}

#[derive(Debug, PartialEq, Serialize)]
pub struct LoginHistory {
    pub user_id: String,
    pub epoch_login: DateTime<Local>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Vote {
    pub user_id: String,
    pub theme_id: i32,
    pub answer_id: i32,
    pub score: i32,
}