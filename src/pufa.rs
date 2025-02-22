pub mod redis_cache;
pub mod rwlock_cache;

use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use regex::Regex;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PufaError {
    #[error("Cannot get uuid")]
    UuidRequest,
    #[error("Cannot parse uuid")]
    UuidParse,
    #[error("Cannot get pufa word")]
    WordRequest,
    #[error("Cannot parse pufa word")]
    WordParse,
    #[error("Cannot get from cache")]
    CacheGet,
}

pub struct Client {}

impl Client {
    pub async fn get_result() -> Result<String, PufaError> {
        let result: String = Self::get_current_uuid().await?;
        let uuid: String = Self::get_uuid_from_js(&result)?;
        let date: String = chrono::offset::Utc::now().format("%Y-%m-%d").to_string();
        Self::get_current_word(&uuid, &date).await
    }

    async fn get_current_uuid() -> Result<String, PufaError> {
        reqwest::get("https://pufa.afup.org/js/instanceConfiguration.js")
            .await
            .map_err(|_| PufaError::UuidRequest)?
            .text()
            .await
            .map_err(|_| PufaError::UuidParse)
    }

    async fn get_current_word(uuid: &str, date: &str) -> Result<String, PufaError> {
        let base64_encoded_token: String = BASE64_STANDARD.encode(format!("{uuid}-{date}"));
        let url: String = format!("https://pufa.afup.org/mots/{base64_encoded_token}.txt");

        reqwest::get(url)
            .await
            .map_err(|_| PufaError::WordRequest)?
            .text()
            .await
            .map_err(|_| PufaError::WordParse)
    }

    fn get_uuid_from_js(js_code: &str) -> Result<String, PufaError> {
        let matched = Regex::new(
            r"[0-9a-fA-F]{8}\b-[0-9a-fA-F]{4}\b-[0-9a-fA-F]{4}\b-[0-9a-fA-F]{4}\b-[0-9a-fA-F]{12}",
        )
            .unwrap()
            .find(js_code);

        matched.map_or(Err(PufaError::UuidRequest), |value| {
            Ok(value.as_str().to_string())
        })
    }
}

pub mod cache {
    use chrono::serde::ts_seconds;
    use chrono::{DateTime, Utc};
    use serde::{Deserialize, Serialize};
    use std::future::Future;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct State {
        pub last_word: String,
        #[serde(with = "ts_seconds")]
        pub updated_at: DateTime<Utc>,
    }

    impl State {
        pub const fn new(last_word: String, updated_at: DateTime<Utc>) -> Self {
            Self {
                last_word,
                updated_at,
            }
        }

        pub fn get_formatted_updated_at(&self) -> String {
            self.updated_at.to_rfc3339()
        }

        pub fn set_current_updated_at(&mut self) {
            self.updated_at = Utc::now();
        }

        pub fn set_last_word(&mut self, new_word: &str) {
            self.last_word = new_word.to_string();
        }
    }

    pub trait Cacheable {
        fn has(&self) -> impl Future<Output = bool> + Send;
        fn is_actual(&self, allowed_diff_seconds: u64) -> impl Future<Output = bool> + Send;
        fn get(&self) -> impl Future<Output = Option<State>> + Send;
        fn set(&self, new_word: String) -> impl Future<Output = State> + Send;
    }

}