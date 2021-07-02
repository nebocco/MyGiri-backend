pub use sql_client::models::Answer;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct AnswerDTO {
    pub user_id: String,
    pub theme_id: i32,
    pub answer_text: String,
}