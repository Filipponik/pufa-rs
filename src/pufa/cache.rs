use chrono::{DateTime, Utc};
use lazy_static::lazy_static;
use tokio::sync::RwLock;
use crate::pufa::main;
use crate::pufa::main::PufaError;

#[derive(Debug, Clone)]
pub struct CacheState {
    just_started: bool,
    last_word: String,
    updated_at: DateTime<Utc>,
}

impl CacheState {
    fn get_formatted_updated_at(&self) -> String {
        self.updated_at.to_rfc3339().to_string()
    }

    fn set_current_updated_at(&mut self) {
        self.updated_at = chrono::offset::Utc::now();
    }

    fn set_last_word(&mut self, new_word: &str) {
        self.just_started = false;
        self.last_word = new_word.to_string();
    }
}

lazy_static! {
    static ref STATE: RwLock<CacheState> = RwLock::new(CacheState {
        just_started: true,
        last_word: "".to_string(),
        updated_at: chrono::offset::Utc::now(),
    });
}

pub async fn get_cached_pufa_word() -> Result<CacheState, PufaError> {
    let read_lock = STATE.read().await;
    let seconds_diff: i64 =
        chrono::offset::Utc::now().timestamp() - read_lock.updated_at.timestamp();

    if !&read_lock.just_started && seconds_diff < 60 {
        return Ok(read_lock.clone());
    }
    drop(read_lock);

    let pufa_word = main::get_result().await;
    match pufa_word {
        Err(error) => Err(error),
        Ok(word) => {
            let mut write_lock = STATE.write().await;
            write_lock.set_current_updated_at();
            write_lock.set_last_word(&word);
            write_lock.set_last_word(&word);

            Ok(write_lock.clone())
        }
    }
}
