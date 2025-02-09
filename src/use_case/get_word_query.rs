use crate::pufa::cache::{Cache, State};
use crate::pufa::main::{Client, PufaError};

pub struct Query {
    cache_ttl: u64,
}

impl Query {
    #[must_use]
    pub const fn new(cache_ttl: u64) -> Self {
        Self { cache_ttl }
    }
}

pub struct Handler {
    query: Query,
}

impl Handler {
    #[must_use]
    pub const fn new(query: Query) -> Self {
        Self { query }
    }

    /// # Errors
    ///
    /// Will return `PufaError` if cannot get pufa word
    pub async fn handle(&self) -> Result<State, PufaError> {
        if Cache::has().await && Cache::is_actual(self.query.cache_ttl).await {
            return Ok(Cache::get().await);
        }

        let pufa_word = Client::get_result().await;
        match pufa_word {
            Err(error) => Err(error),
            Ok(word) => Ok(Cache::set(word).await),
        }
    }
}
