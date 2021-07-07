use serde::Deserialize;
pub use sql_client::models::Theme;

#[derive(Deserialize)]
pub struct ThemeDTO {
    pub user_id: String,
    pub theme_text: String
}