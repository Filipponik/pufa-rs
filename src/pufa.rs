use crate::pufa::cache::CacheState;

mod cache;
mod main;

pub async fn get_cached_result() -> Result<CacheState, main::PufaError> {
    cache::get_cached_pufa_word().await
}
