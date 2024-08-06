use std::collections::HashMap;
use std::env;
use std::sync::Arc;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use tokio::sync::{mpsc, oneshot, RwLock};
use rand::rngs::OsRng;
use ed25519_dalek::{Signature, Signer, SigningKey};
use reqwest::Client;
use dotenvy::dotenv;

use crate::config::Config;
use crate::tee::model::{Operator, Params, OperatorReq, WorkerStatus, OperatorResp, AnswerReq};
use crate::opml::model::{OpmlAnswer, OpmlRequest};

#[derive(Debug, Clone)]
pub struct Server {
    pub sign_key: SigningKey,
    pub tee_operator_collections: HashMap<String, Operator>,
    pub pg: Pool<ConnectionManager<PgConnection>>,
    pub tee_channels: HashMap<String, mpsc::Sender<AnswerReq>>,
    pub opml_channels: HashMap<String, mpsc::Sender<OpmlAnswer>>,
}

#[derive(Debug, Clone)]
pub struct SharedState(pub(crate) Arc<RwLock<Server>>);

impl SharedState {
    pub async fn new(config: Config) -> Self {
        let server = Server::new(config).await;
        SharedState(Arc::new(RwLock::new(server)))
    }
}

impl Server {
    pub async fn new(config: Config) -> Self {
        let mut csprng = OsRng;
        let sign_key = SigningKey::generate(&mut csprng);
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pg = Pool::builder().build(manager).expect("Failed to create pool.");

        Self {
            sign_key,
            tee_operator_collections: Default::default(),
            pg,
            tee_channels: Default::default(),
            opml_channels: Default::default(),
        }
    }

    pub fn sign(&self, message: &[u8]) -> Signature {
        self.sign_key.sign(message)
    }

    pub fn verify(&self, message: &[u8], signature: &Signature) -> bool {
        self.sign_key.verify(message, signature).is_ok()
    }

    pub fn add_worker(&mut self, worker_name: String, check_heart_beat: bool, worker_status: WorkerStatus, multimodal: bool) {
        let worker_name_clone = worker_name.clone();
        let operator = Operator {
            worker_name: worker_name_clone,
            check_heart_beat,
            worker_status,
            multimodal,
        };
        self.tee_operator_collections.insert(worker_name, operator);
    }

    pub async fn send_tee_inductive_task(&self, worker_name: String, req: OperatorReq) -> OperatorResp {
        let operator = self.tee_operator_collections.get(&worker_name).unwrap();
        let op_url = format!("{}/api/v1/question", operator.worker_name);
        //let client = Client::builder().proxy(reqwest::Proxy::http("http://127.0.0.1:8080")?).build().unwrap();
        let resp = Client::new()
            .post(op_url)
            .json(&req)
            .send()
            .await
            .unwrap();

        resp.json::<OperatorResp>().await.unwrap()
    }

    pub async fn send_opml_request(&self, req: OpmlRequest) -> Result<(), Box<dyn std::error::Error>> {
        tracing::info!("Sending opml request {:?}", req);
        let client = reqwest::Client::new();
        let opml_server_url = format!("{}/api/v1/question", "http://127.0.0.1:1234");

        let response = client
            .post(opml_server_url)
            .json(&req)
            .send()
            .await?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(format!("OPML server responded with status: {}", response.status()).into())
        }
    }
}

pub async fn sign_handler() -> String {
    let mut csprng = OsRng;
    let key = SigningKey::generate(&mut csprng);
    let message: &[u8] = b"Hello, World!";
    let signature = key.sign(message);
    signature.to_string()
}