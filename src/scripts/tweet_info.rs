use crate::{
    config::db::Pool,
    models::theme::Theme
};

use surf::{ Response, Error, http::Method };
type Result<T> = std::result::Result<T, Error>; 

use std::collections::BTreeMap;

pub async fn tweet_info(pool: &Pool) {
    let theme = get_new_theme(&pool).await.expect("Failed to load theme.");
    if let Some(theme) = theme {
        let client = Client::from_env().expect("Failed to load env values.");
        let text = generate_text(theme);
        let mut response = client.tweet(&text).await.expect("Failed to tweet.");
        let res_text = response.body_string().await.expect("Failed to read response.");
        log::info!("{}", res_text);
    }
}

struct Client {
    api_key: String,
    api_secret_key: String,
    access_token: String,
    access_token_secret: String,
}

impl Client {
    fn from_env() -> Result<Client> {
        let api_key = std::env::var("TWITTER_API_KEY")?;
        let api_secret_key = std::env::var("TWITTER_API_SECRET_KEY")?;
        let access_token = std::env::var("TWITTER_ACCESS_TOKEN")?;
        let access_token_secret = std::env::var("TWITTER_ACCESS_TOKEN_SECRET")?;
        Ok(Client {
            api_key,
            api_secret_key,
            access_token,
            access_token_secret,
        })
    }

    async fn tweet(&self, status: &str) -> Result<Response> {
        let mut parameters = BTreeMap::new();
        parameters.insert("status", status);
        self.request(
            Method::Post,
            "https://api.twitter.com/1.1/statuses/update.json",
            &parameters,
        ).await
    }

    async fn request(
        &self,
        method: Method,
        url: &str,
        parameters: &BTreeMap<&str, &str>,
    ) -> Result<Response> {
        let url_with_parameters = surf::Url::parse(format!(
            "{}?{}",
            url,
            equal_collect(parameters.iter().map(|(key, value)| { (*key, *value) })).join("&")
        ).as_ref())?;

        let request = surf::Request::builder(method, url_with_parameters)
            .header(
                "Authorization",
                self.authorization(&method, url, parameters)
            )
            .content_type("application/x-www-form-urlencoded")
            .build();

        surf::client().send(request).await
    }

    fn authorization(
        &self,
        method: &Method,
        url: &str,
        parameters: &BTreeMap<&str, &str>,
    ) -> String {
        let timestamp = format!("{}", chrono::Utc::now().timestamp());
        let nonce: String = {
            use rand::prelude::*;
            let mut rng = thread_rng();
            (0..32)
                .map(|_| rng.sample(rand::distributions::Alphanumeric))
                .map(char::from)
                .collect()
        };

        let mut other_parameters: Vec<(&str, &str)> = vec![
            ("oauth_consumer_key", &self.api_key),
            ("oauth_token", &self.access_token),
            ("oauth_signature_method", "HMAC-SHA1"),
            ("oauth_version", "1.0"),
            ("oauth_timestamp", &timestamp),
            ("oauth_nonce", &nonce),
        ];

        let signature = self.signature(method, url, parameters.clone(), &other_parameters);

        other_parameters.push(("oauth_signature", &signature));

        format!(
            "OAuth {}",
            equal_collect(other_parameters.into_iter()).join(", ")
        )
    }

    fn signature<'a>(
        &self,
        method: &Method,
        url: &str,
        mut parameters: BTreeMap<&'a str, &'a str>,
        other_parameters: &Vec<(&'a str, &'a str)>,
    ) -> String {
        for (key, value) in other_parameters {
            parameters.insert(key, value);
        }
        let parameter_string = equal_collect(parameters.into_iter()).join("&");

        let signature_base_string = format!(
            "{}&{}&{}",
            method,
            percent_encode(url),
            percent_encode(&parameter_string)
        );
        let signing_key = format!("{}&{}", self.api_secret_key, self.access_token_secret);
        base64::encode(hmacsha1::hmac_sha1(
            signing_key.as_bytes(),
            signature_base_string.as_bytes(),
        ))
    }
}

fn equal_collect<'a, T: Iterator<Item = (&'a str, &'a str)>>(iter: T) -> Vec<String> {
    iter.map(|(key, value)| format!("{}={}", percent_encode(key), percent_encode(value)))
        .collect()
}

fn percent_encode(s: &str) -> percent_encoding::PercentEncode {
    use percent_encoding::*;
    const FRAGMENT: &AsciiSet = &NON_ALPHANUMERIC
        .remove(b'*')
        .remove(b'-')
        .remove(b'.')
        .remove(b'_');
    utf8_percent_encode(s, FRAGMENT)
}

async fn get_new_theme(pool: &Pool) -> Result<Option<Theme>> {
    use sql_client::theme_client::ThemeClient;

    let now = chrono::Local::now();
    let themes = pool.get_themes_active(now).await?;
    let theme = match themes.into_iter().max_by_key(|t| t.epoch_open) {
        Some(theme) => theme,
        None => { return Ok(None); }
    };
    let theme = if theme.epoch_open > now - chrono::Duration::minutes(30) {
        Some(theme)
    } else {
        None
    };
    Ok(theme)
}

fn generate_text(theme: Theme) -> String {
    let theme_text = theme.theme_text;
    let deadline = theme.epoch_open + chrono::Duration::hours(24);
    let theme_url = format!("https://mygiri.vercel.app/theme/{}", theme.id.unwrap());
    let text = format!(
        "お題が公開されました\n『{}』\n\n回答締め切りは{}です🍵\n{}",
        theme_text, deadline.format("%-m月%-d日 %-H時"), theme_url
    );
    text
}

#[cfg(test)]
mod test {
    #[test]
    fn test_generate_text() {
        use chrono::TimeZone;
        use super::*;

        let theme = Theme {
            id: Some(23),
            user_id: "".to_string(),
            display_name: None,
            theme_text: "テーマ テキスト".to_string(),
            epoch_open: chrono::Local.ymd(2008, 12, 3).and_hms(20, 00, 00)
        };

        let text = generate_text(theme);
        println!("{}", text);
        assert_eq!(text,
"お題が公開されました
『テーマ テキスト』

回答締め切りは12月4日 20時です🍵
https://mygiri.vercel.app/theme/23"
        );
    }
}