use serde::{Deserialize};

pub use sql_client::models::User;

#[derive(Deserialize)]
pub struct UserDTO {
    pub user_id: String,
    pub password: String,
    pub display_name: Option<String>,
}

#[derive(Deserialize)]
pub struct UserNameData {
    pub user_id: String,
    pub display_name: Option<String>,
}