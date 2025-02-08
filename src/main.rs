use tracing::error;

mod config;
mod pufa;
mod server;

#[tokio::main]
async fn main() {
    config::setup_tracing();

    if let Err(e) = server::start().await {
        error!("{e}");
    }
}
