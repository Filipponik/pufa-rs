use crate::pufa::cache::CacheState;

mod main;
mod cache;

pub async fn get_result() -> Result<String, main::PufaError> {
    main::get_result().await
}

pub async fn get_cached_result() -> Result<CacheState, main::PufaError> {
    cache::get_cached_pufa_word().await
}