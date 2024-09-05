use aos_dispatcher::config::CustomConfig;
use aos_dispatcher::opml::handler::*;
use aos_dispatcher::server::server::SharedState;
use aos_dispatcher::service::nostr::model::JobAnswer;
use aos_dispatcher::tee::handler::*;
use aos_dispatcher::ws;
use axum::error_handling::HandleErrorLayer;
use axum::handler::Handler;
use axum::http::Method;
use axum::routing::get;
use axum::{routing::post, Router};
use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;
use std::time::Duration;
use tokio::sync::mpsc;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

use aos_dispatcher::service;
use aos_dispatcher::{job, operator};
use tower_http::cors::{Any, CorsLayer};
use tracing::Level;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    let custom_config = CustomConfig::from_toml().await;
    let config = aos_dispatcher::config::Config::new().merge(&custom_config);
    let max_level = if let Some(cl) = &custom_config.log_level {
        tracing::Level::from_str(&cl).unwrap_or(Level::INFO)
    } else {
        Level::INFO
    };
    let addr = format!("{}:{}", config.server.host, config.server.port);
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_max_level(max_level)
        .init();
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(Any);

    let (dispatch_task_tx, dispatch_task_rx) = mpsc::channel::<u32>(200);

    let (job_status_tx, job_status_rx) = mpsc::channel::<JobAnswer>(200);

    let secret_key = config.secret_key;

    let mut server =
        SharedState::new(config, dispatch_task_tx.clone(), job_status_tx.clone()).await;

    // let nostr_sub_task = tokio::spawn(aos_dispatcher::service::nostr::subscription_service(
    //     server.clone(),
    //     job_status_rx,
    //     dispatch_task_tx.clone(),
    //     secret_key,
    //     custom_config.default_relay.unwrap_or("ws://localhost:8080".into())
    // ));

    let dispatch_task = tokio::spawn(service::task::dispatch_task(
        server.clone(),
        dispatch_task_rx,
    ));

    // build our application with a single route
    let app = Router::new()
        .route("/ping", get(|| async { "pong" }))
        .route("/sign", post(sign))
        .route("/register_worker", post(register_worker))
        .route("/receive_heart_beat", post(receive_heart_beat))
        .route("/api/question", post(tee_question_handler))
        .route("/api/operator/register", post(operator::handler::register))
        .route("/api/operator/info", post(operator::handler::operator_info))
        .route("/api/job/submit", post(job::handler::submit_job))
        .route("/api/job/result", post(job::handler::query_job_result))
        .route("/api/tee_callback", post(tee_callback))
        .route("/api/opml_question", post(opml_question_handler))
        .route("/api/opml_callback", post(opml_callback))
        .route("/api/list_models", post(list_models))
        .route("/admin/list_workers", post(list_workers))
        .route("/admin/list_questions", post(list_questions_handler))
        .route("/admin/list_answers", post(list_answers_handler))
        .route("/ws", get(ws::handler))
        .with_state(server)
        .layer(cors)
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(handle_error))
                .timeout(Duration::from_secs(600))
                .layer(TraceLayer::new_for_http()),
        );

    let server_task = tokio::spawn(async {
        tracing::info!("start server on {}", addr);
        match tokio::net::TcpListener::bind(addr).await {
            Ok(listener) => {
                match axum::serve(
                    listener,
                    app.into_make_service_with_connect_info::<SocketAddr>(),
                )
                .await {
                    Ok(_) => {},
                    Err(e) => {
                        tracing::error!("start server error: {}", e);
                    },
                }
            }
            Err(e) => {
                tracing::error!("start server error: {}", e);
            }
        }
    });

    let _ = tokio::join!(
        // nostr_sub_task, 
        server_task, 
        dispatch_task,
    );
}
