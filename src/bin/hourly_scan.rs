use actix_backend::scripts;
use sql_client::create_pool;

#[async_std::main]
async fn main() {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL is not set.");
    let pool = create_pool(&database_url).await
        .expect("Failed to initialize the connection pool");

    scripts::update_profile(&pool).await;
    scripts::tweet_info(&pool).await;
}