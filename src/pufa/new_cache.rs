use crate::pufa::cache::{Cacheable, State};
use chrono::Utc;
use once_cell::sync::Lazy;
use tokio::sync::RwLock;

static STATE: Lazy<RwLock<Option<State>>> = Lazy::new(|| RwLock::new(None));

pub struct NewCache;

impl Cacheable for NewCache {
    async fn has(&self) -> bool {
        STATE.read().await.is_some()
    }

    async fn is_actual(&self, allowed_diff_seconds: u64) -> bool {
        match &*STATE.read().await {
            None => false,
            Some(state) => {
                Utc::now()
                    .timestamp()
                    .abs_diff(state.updated_at.timestamp())
                    < allowed_diff_seconds
            }
        }
    }

    async fn get(&self) -> Option<State> {
        STATE.read().await.clone()
    }

    async fn set(&self, new_word: String) -> State {
        let mut write_lock = STATE.write().await;
        let state = State::new(new_word, Utc::now());
        *write_lock = Some(state.clone());

        state
    }
}
