use sql_client::{
    theme_client::ThemeClient,
    profile_client::ProfileClient
};
use crate::config::db::Pool;
use std::{ thread, time };

pub async fn update_profile(pool: &Pool) {
    let threshold = chrono::Local::now() - chrono::Duration::hours(32);
    let themes = pool.get_themes_to_update(threshold).await.unwrap();

    log::info!("Start updating");
    for theme in themes {
        let theme_id = theme.id.unwrap();
        log::info!("Updating theme {}", theme_id);
        match pool.update_profile(theme).await {
            Ok(_) => { log::info!("Updated theme {} successfully", theme_id); },
            Err(_) => { log::info!("Error occured"); break }
        }
        sleep_1sec();
    }
}

fn sleep_1sec() {
    thread::sleep(time::Duration::from_millis(1000));
}