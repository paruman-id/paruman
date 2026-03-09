use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainStatus {
    pub chain_id: String,
    pub rpc_url: String,
    pub status: String,
    pub network: String,
}

pub async fn chain_status() -> Json<ChainStatus> {
    Json(ChainStatus {
        chain_id: "paseo".to_string(),
        rpc_url: "wss://rpc1.paseo.mandalachain.io".to_string(),
        status: "connected".to_string(),
        network: "MandalaChain Paseo Testnet".to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_chain_status() {
        let status = chain_status().await;
        assert_eq!(status.chain_id, "paseo");
        assert_eq!(status.network, "MandalaChain Paseo Testnet");
    }
}
