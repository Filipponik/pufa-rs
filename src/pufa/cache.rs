use chrono::{DateTime, Utc};
use lazy_static::lazy_static;
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
pub struct State {
    just_started: bool,
    pub last_word: String,
    pub updated_at: DateTime<Utc>,
}

impl State {
    pub fn get_formatted_updated_at(&self) -> String {
        self.updated_at.to_rfc3339()
    }

    fn set_current_updated_at(&mut self) {
        self.updated_at = Utc::now();
    }

    fn set_last_word(&mut self, new_word: &str) {
        self.just_started = false;
        self.last_word = new_word.to_string();
    }
}

lazy_static! {
    static ref STATE: RwLock<State> = RwLock::new(State {
        just_started: true,
        last_word: String::new(),
        updated_at: Utc::now(),
    });
}

pub struct Cache;

impl Cache {
    pub async fn has() -> bool {
        let read_lock = STATE.read().await;
        !read_lock.just_started
    }

    pub async fn is_actual(allowed_diff_seconds: u64) -> bool {
        let read_lock = STATE.read().await;

        Utc::now()
            .timestamp()
            .abs_diff(read_lock.updated_at.timestamp())
            < allowed_diff_seconds
    }

    pub async fn get() -> State {
        let read_lock = STATE.read().await;
        read_lock.clone()
    }

    pub async fn set(new_word: String) -> State {
        let mut write_lock = STATE.write().await;
        write_lock.set_current_updated_at();
        write_lock.set_last_word(&new_word);
        write_lock.clone()
    }
}
