use crate::pufa::cache::{Cacheable, State};
use chrono::Utc;
use lazy_static::lazy_static;
use tokio::sync::RwLock;

lazy_static! {
    static ref STATE: RwLock<State> = RwLock::new(State {
        last_word: String::new(),
        updated_at: Utc::now(),
    });
}

pub struct OldCache;

impl Cacheable for OldCache {
    async fn has(&self) -> bool {
        let read_lock = STATE.read().await;
        !read_lock.last_word.is_empty()
    }

    async fn is_actual(&self, allowed_diff_seconds: u64) -> bool {
        let read_lock = STATE.read().await;

        Utc::now()
            .timestamp()
            .abs_diff(read_lock.updated_at.timestamp())
            < allowed_diff_seconds
    }

    async fn get(&self) -> Option<State> {
        let read_lock = STATE.read().await;
        Some(read_lock.clone())
    }

    async fn set(&self, new_word: String) -> State {
        let mut write_lock = STATE.write().await;
        write_lock.set_current_updated_at();
        write_lock.set_last_word(&new_word);
        write_lock.clone()
    }
}
