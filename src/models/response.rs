// Copyright (c) 2018 Ba Hai Phan
// Code released under the MIT license
// https://opensource.org/licenses/mit-license.php

use serde::Serialize;

#[derive(Serialize)]
pub struct ResponseBody<T> {
    pub message: String,
    pub data: T,
}

impl<T> ResponseBody<T> {
    pub fn new(message: &str, data: T) -> ResponseBody<T> {
        ResponseBody {
            message: message.to_string(),
            data,
        }
    }
}