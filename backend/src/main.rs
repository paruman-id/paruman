mod config;
mod blockchain;

use axum::{
    routing::get,
    Router,
};
use std::net::SocketAddr;
use tracing_subscriber;
use config::Config;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    let config = Config::from_env();
    tracing::info!("Starting Paruman backend");
    tracing::info!("MandalaChain RPC: {}", config.mandalachain.rpc_url);
    tracing::info!("Database: {}", config.database.url);

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/chain/status", get(blockchain::chain_status));

    let addr = SocketAddr::from(([
        config.server.host.parse::<u8>().unwrap_or(127),
        0,
        0,
        1,
    ], config.server.port));
    tracing::info!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind to address");

    axum::serve(listener, app)
        .await
        .expect("Server error");
}

async fn health_check() -> &'static str {
    "OK"
}
