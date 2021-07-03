use serde::{Deserialize};
pub use sql_client::models::Vote;

#[derive(Deserialize)]
pub struct VoteRequest {
    pub user_id: String,
    pub theme_id: i32,
    pub votes: Vec<Vote>
}