use crate::pufa::cache::State;

mod cache;
mod main;

pub async fn get_cached_result() -> Result<State, main::PufaError> {
    cache::get_cached_pufa_word().await
}
