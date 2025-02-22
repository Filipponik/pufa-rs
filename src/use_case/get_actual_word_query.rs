use crate::pufa::{Client, PufaError};

pub struct Query;

pub struct Handler {
    #[allow(dead_code)]
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
    pub async fn handle(&self) -> Result<String, PufaError> {
        Client::get_result().await
    }
}
