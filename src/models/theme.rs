use serde::Deserialize;
pub use sql_client::models::Theme;

#[derive(Deserialize)]
pub struct ThemeDTO {
    pub author: String,
    pub theme_text: String
}