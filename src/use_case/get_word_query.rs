use crate::pufa::cache::{Cacheable, State};
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
    pub async fn handle<T: Cacheable>(&self, cache: T) -> Result<State, PufaError> {
        if cache.has().await && cache.is_actual(self.query.cache_ttl).await {
            return match cache.get().await {
                Some(state) => Ok(state),
                None => Err(PufaError::CacheGet)
            };
        }

        let pufa_word = Client::get_result().await;
        match pufa_word {
            Err(error) => Err(error),
            Ok(word) => Ok(cache.set(word).await),
        }
    }
}
