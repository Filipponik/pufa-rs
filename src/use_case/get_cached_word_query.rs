use crate::pufa::cache::{Cacheable, State};
use crate::pufa::{Client, PufaError};

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
    pub async fn handle<T: Cacheable + Send>(&self, cache: T) -> Result<State, PufaError> {
        if cache.has().await && cache.is_actual(self.query.cache_ttl).await {
            return cache.get().await.ok_or(PufaError::CacheGet);
        }

        let pufa_word = Client::get_word().await;
        match pufa_word {
            Err(error) => Err(error),
            Ok(word) => Ok(cache.set(word).await),
        }
    }
}
