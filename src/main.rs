mod pufa;
mod server;
mod config;


#[tokio::main]
async fn main() {
    config::setup_tracing();
    server::start_server().await;
}