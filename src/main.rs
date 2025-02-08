use thiserror::Error;

mod pufa;
mod server;
mod config;

#[derive(Debug, Error)]
enum AppError {
    #[error("App error: {0}")]
    Server(#[from] server::Error),
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
    config::setup_tracing();
    server::start_server().await?;

    Ok(())
}