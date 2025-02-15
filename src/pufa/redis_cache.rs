use crate::pufa::cache::{Cacheable, State};
use chrono::Utc;
use redis::aio::MultiplexedConnection;
use redis::AsyncCommands;

#[derive(Clone)]
pub struct RedisCache {
    conn: String,
}

impl RedisCache {
    const KEY: &'static str = "current_pufa_word";

    #[allow(dead_code)]
    pub const fn new(conn: String) -> Self {
        Self { conn }
    }

    pub async fn connect(&self) -> MultiplexedConnection {
        let client = redis::Client::open(self.conn.to_string()).unwrap();
        client.get_multiplexed_async_connection().await.unwrap()
    }
}

impl Cacheable for RedisCache {
    async fn has(&self) -> bool {
        let mut con = self.connect().await;
        let val: Option<String> = con.get(Self::KEY).await.unwrap();
        val.is_some()
    }

    async fn is_actual(&self, allowed_diff_seconds: u64) -> bool {
        let mut con = self.connect().await;
        let val: Option<String> = con.get(Self::KEY).await.unwrap();

        val.is_some_and(|value| {
            serde_json::from_str::<State>(&value).is_ok_and(|state| {
                Utc::now()
                    .timestamp()
                    .abs_diff(state.updated_at.timestamp())
                    < allowed_diff_seconds
            })
        })
    }

    async fn get(&self) -> Option<State> {
        let mut con = self.connect().await;
        con.get::<&str, Option<String>>(Self::KEY)
            .await
            .unwrap()
            .and_then(|value| serde_json::from_str::<State>(&value).ok())
    }

    async fn set(&self, new_word: String) -> State {
        let mut con = self.connect().await;
        let state = State::new(new_word, Utc::now());
        let serialized = serde_json::to_string(&state).unwrap();
        let _: () = con.set(Self::KEY, serialized).await.unwrap();

        state
    }
}
