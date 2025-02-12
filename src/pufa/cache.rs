use chrono::{DateTime, Utc};
use std::future::Future;
use serde::{Deserialize, Serialize};
use chrono::serde::ts_seconds;

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
