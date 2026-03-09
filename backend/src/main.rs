mod config;
mod blockchain;
mod identity;

use axum::{
    routing::{get, post},
    Router,
    Json,
};
use std::net::SocketAddr;
use tracing_subscriber;
use config::Config;
use identity::{IdentityVerificationRequest, IdentityVerificationResponse};

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
        .route("/chain/status", get(blockchain::chain_status))
        .route("/identity/verify", post(identity_verify));

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

async fn identity_verify(
    Json(req): Json<IdentityVerificationRequest>,
) -> Json<IdentityVerificationResponse> {
    let user_id = uuid::Uuid::new_v4();
    let mandala_id = identity::MandalaID {
        did: req.mandala_id_did.clone(),
        chain_anchor_hash: None,
        verified_at: Some(chrono::Utc::now()),
    };

    let verification = identity::IdentityVerification::new(user_id, mandala_id);

    tracing::info!(
        "Identity verification: user={}, tier={:?}, kawenang_active={}",
        user_id,
        verification.identity_tier,
        verification.is_kawenang_active()
    );

    Json(IdentityVerificationResponse {
        user_id,
        identity_tier: verification.identity_tier,
        is_kawenang_active: verification.is_kawenang_active(),
        days_until_active: verification.days_until_kawenang_active(),
        verified_at: verification.verified_at,
    })
}
