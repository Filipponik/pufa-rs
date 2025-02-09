use tracing::error;

mod config;
mod pufa;
mod server;
pub mod use_case;

#[tokio::main]
async fn main() {
    config::setup_tracing();

    if let Err(e) = server::start().await {
        error!("{e}");
    }
}
