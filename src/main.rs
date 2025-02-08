use tracing::error;

mod pufa;
mod server;
mod config;

#[tokio::main]
async fn main() {
    config::setup_tracing();

    if let Err(e) = server::start_server().await {
        error!("{e}")
    }
}