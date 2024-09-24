use alloy::primitives::keccak256;
use alloy::signers::local::PrivateKeySigner;
use axum::extract::ws::Message;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use dotenvy::dotenv;
use ed25519_dalek::{SecretKey, Signature, Signer, SigningKey};
use rand::rngs::OsRng;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};

use crate::config::Config;
use crate::service::nostr::model::JobAnswer;
use crate::tee::model::{AnswerReq, Operator};

#[derive(Debug, Clone)]
pub struct Server {
    pub config: Config,
    pub sign_key: SigningKey,
    pub nostr_keys: nostr::Keys,
    pub ecdsa_signer: PrivateKeySigner,
    pub tee_operator_collections: HashMap<String, Operator>,
    pub pg: Pool<ConnectionManager<PgConnection>>,
    pub tee_channels: HashMap<String, mpsc::Sender<AnswerReq>>,
    pub worker_channels: HashMap<String, mpsc::Sender<Message>>,
    pub operator_channels: HashMap<String, mpsc::Sender<Message>>,
    pub dispatch_task_tx: Option<mpsc::Sender<u32>>,
    pub job_status_tx: Option<mpsc::Sender<JobAnswer>>,
}

#[derive(Debug, Clone)]
pub struct SharedState(pub(crate) Arc<RwLock<Server>>);

impl SharedState {
    pub async fn new(
        config: Config,
        dispatch_task_tx: mpsc::Sender<u32>,
        job_status_tx: mpsc::Sender<JobAnswer>,
    ) -> Self {
        let server = Server::new(config, dispatch_task_tx, job_status_tx).await;
        SharedState(Arc::new(RwLock::new(server)))
    }
}

use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();
fn run_migration(conn: &mut PgConnection) {
    conn.run_pending_migrations(MIGRATIONS).unwrap();
}

impl Server {
    pub async fn new(
        config: Config,
        dispatch_task_tx: mpsc::Sender<u32>,
        job_status_tx: mpsc::Sender<JobAnswer>,
    ) -> Self {
        // let mut csprng = OsRng;
        // let sign_key = SigningKey::generate(&mut csprng);
        let secret_key: SecretKey = config.secret_key;
        let sign_key = SigningKey::from(secret_key);
        let ecdsa_signer = PrivateKeySigner::from_slice(&secret_key).expect("error ecdsa singer");
        let nostr_keys = nostr::Keys::new(nostr::SecretKey::from_slice(&secret_key).unwrap());
        dotenv().ok();

        let db_url = config.custom_config.db.clone().and_then(|db| db.url);

        let database_url = db_url.expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pg = Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");

        let mut conn = pg.get().expect("Failed to get conn.");

        run_migration(&mut conn);

        Self {
            config,
            sign_key,
            nostr_keys,
            ecdsa_signer,
            tee_operator_collections: Default::default(),
            pg,
            tee_channels: Default::default(),
            worker_channels: Default::default(),
            operator_channels: Default::default(),
            dispatch_task_tx: Some(dispatch_task_tx),
            job_status_tx: Some(job_status_tx),
        }
    }

    pub fn sign(&self, message: &[u8]) -> Signature {
        self.sign_key.sign(message)
    }

    pub fn verify(&self, message: &[u8], signature: &Signature) -> bool {
        self.sign_key.verify(message, signature).is_ok()
    }

    pub fn ecdsa_sign(&self, message: &[u8]) -> Signature {
        self.sign_key.sign(keccak256(message).as_slice())
    }

    pub fn ecdsa_verify(&self, message: &[u8], signature: Signature) -> anyhow::Result<()> {
        self.sign_key.verify(message, &signature)?;
        Ok(())
    }
}

pub async fn sign_handler() -> String {
    let mut csprng = OsRng;
    let key = SigningKey::generate(&mut csprng);
    let message: &[u8] = b"Hello, World!";
    let signature = key.sign(message);
    signature.to_string()
}
