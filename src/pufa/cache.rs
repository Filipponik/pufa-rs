use std::future::Future;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct State {
    pub last_word: String,
    pub updated_at: DateTime<Utc>,
}

impl State {
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
    fn get(&self) -> impl Future<Output = State> + Send;
    fn set(&self, new_word: String) -> impl Future<Output = State> + Send;
}
