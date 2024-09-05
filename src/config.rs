use ed25519_dalek::SecretKey;
use nostr::nips::nip06::FromMnemonic;
use nostr_sdk::Keys;
use serde::Deserialize;
#[derive(Debug, Deserialize, Default, Clone)]
pub struct CustomConfig {
    pub log_level: Option<String>,
    pub address: Option<String>,
    pub port: Option<u16>,
    pub mnemonic: Option<String>,
    pub default_relay: Option<String>,
}
impl CustomConfig {
    pub async fn from_toml() -> Self {
        let f = tokio::fs::read_to_string("dispatcher.toml").await;
        let custom = match f {
            Ok(s) => {
                let custom_config = match toml::from_str::<CustomConfig>(s.as_str()) {
                    Ok(c) => c,
                    Err(_) => {
                        tracing::error!("parse dispatcher.toml fail");
                        CustomConfig::default()
                    },
                };
                custom_config
            },
            Err(_) => {
                tracing::error!("parse dispatcher.toml fail");
                CustomConfig::default()
            },
        };
        custom
    }
}
#[derive(Debug)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub secret_key: SecretKey,
}

#[derive(Debug)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    // pub log_level: String,
}

#[derive(Debug)]
pub struct DatabaseConfig {
    pub url: String,
}

impl Config {
    pub fn new() -> Self {
        Self {
            server: ServerConfig {
                host: "127.0.0.1".to_string(),
                port: 3000,
            },
            database: DatabaseConfig {
                url: "postgres://postgres:123456@127.0.0.1:5432/dispatcher".to_string(),
            },
            secret_key: SecretKey::default(),
        }
    }

    pub  fn merge(&mut self, custom: &CustomConfig) -> Self {
        let mut config = Self::new();
        config.server.host = custom.address.clone().unwrap_or(config.server.host);
        config.server.port = custom.port.unwrap_or(config.server.port);
        config.secret_key = custom.mnemonic.clone().map_or(config.secret_key, |mnemonic| {
            let pair = Keys::from_mnemonic(mnemonic, None).unwrap();
            pair.secret_key().unwrap().secret_bytes()
        });
        config
    }

}