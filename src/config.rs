use ed25519_dalek::SecretKey;
use nostr::nips::nip06::FromMnemonic;
use nostr_sdk::Keys;
use serde::Deserialize;

#[derive(Debug, Deserialize, Default, Clone)]
pub struct CustomRegister {
    pub endpoint: Option<String>,
    pub contract: Option<String>,
    pub account: Option<String>,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct CustomServer {
    pub host: Option<String>,
    pub port: Option<u16>,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct CustomLog {
    pub level: Option<String>,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct CustomNostr {
    pub relay: Option<String>,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct CustomDb {
    pub url: Option<String>,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct CustomAccount {
    pub mnemonic: Option<String>,
}
#[derive(Debug, Deserialize, Default, Clone)]
pub struct CustomConfig {
    pub log_level: Option<String>,
    pub address: Option<String>,
    pub port: Option<u16>,
    pub mnemonic: Option<String>,
    pub default_relay: Option<String>,
    pub server: Option<CustomServer>,
    pub register: Option<CustomRegister>,
    pub log: Option<CustomLog>,
    pub nostr: Option<CustomNostr>,
    pub account: Option<CustomAccount>,
    pub db: Option<CustomDb>,
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
                    }
                };
                custom_config
            }
            Err(_) => {
                tracing::error!("parse dispatcher.toml fail");
                CustomConfig::default()
            }
        };
        custom
    }
}
#[derive(Debug, Clone)]
pub struct Config {
    pub secret_key: SecretKey,
    pub custom_config: CustomConfig,
}

#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    // pub log_level: String,
}

#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub url: String,
}

impl Config {
    pub fn new() -> Self {
        Self {
            secret_key: SecretKey::default(),
            custom_config: CustomConfig::default(),
        }
    }

    pub fn merge(&mut self, custom: &CustomConfig) -> Self {
        let mut config = Self::new();
        let secret_key = custom
        .account
        .clone()
        .and_then(|a| a.mnemonic)
        .and_then(|mnemonic| {
             Keys::from_mnemonic(mnemonic, None).ok()
        }).and_then(|p| {
            p.secret_key().cloned().ok()
        }).and_then(|s| {
            Some(s.secret_bytes())
        }).unwrap_or(SecretKey::default());
        config.secret_key = secret_key;
        config.custom_config = custom.clone();
        config
    }
}
