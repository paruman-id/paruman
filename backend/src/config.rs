use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub redis: RedisConfig,
    pub s3: S3Config,
    pub mandalachain: MandalachainConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedisConfig {
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct S3Config {
    pub endpoint: String,
    pub access_key: String,
    pub secret_key: String,
    pub bucket: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MandalachainConfig {
    pub rpc_url: String,
    pub chain_id: String,
}

impl Config {
    pub fn from_env() -> Self {
        Config {
            server: ServerConfig {
                host: env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
                port: env::var("SERVER_PORT")
                    .unwrap_or_else(|_| "3000".to_string())
                    .parse()
                    .expect("SERVER_PORT must be a valid u16"),
            },
            database: DatabaseConfig {
                url: env::var("DATABASE_URL")
                    .expect("DATABASE_URL must be set"),
            },
            redis: RedisConfig {
                url: env::var("REDIS_URL")
                    .unwrap_or_else(|_| "redis://localhost:6379".to_string()),
            },
            s3: S3Config {
                endpoint: env::var("S3_ENDPOINT")
                    .unwrap_or_else(|_| "http://localhost:9000".to_string()),
                access_key: env::var("S3_ACCESS_KEY")
                    .unwrap_or_else(|_| "minioadmin".to_string()),
                secret_key: env::var("S3_SECRET_KEY")
                    .unwrap_or_else(|_| "minioadmin".to_string()),
                bucket: env::var("S3_BUCKET")
                    .unwrap_or_else(|_| "paruman".to_string()),
            },
            mandalachain: MandalachainConfig {
                rpc_url: env::var("MANDALACHAIN_RPC_URL")
                    .unwrap_or_else(|_| "wss://rpc1.paseo.mandalachain.io".to_string()),
                chain_id: env::var("MANDALACHAIN_CHAIN_ID")
                    .unwrap_or_else(|_| "paseo".to_string()),
            },
        }
    }
}
