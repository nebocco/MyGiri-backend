use sql_client::{
    create_pool,
    theme_client::ThemeClient,
    profile_client::ProfileClient
};

#[async_std::main]
async fn main() {
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set.");
    let pool = create_pool(url).await.unwrap();
    let threshold = chrono::Local::now() - chrono::Duration::hours(32);
    let themes = pool.get_themes_to_update(threshold).await.unwrap();
    for theme in themes {
        pool.update_profile(theme.id.unwrap()).await.unwrap();
    }
}