use std::time::Duration;
use axum::{
    Router,
    routing::post,
};
use axum::error_handling::HandleErrorLayer;
use axum::handler::Handler;
use axum::http::Method;
use axum::routing::get;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use aos_dispatcher::tee::handler::*;
use aos_dispatcher::server::server::SharedState;
use aos_dispatcher::opml::handler::*;

use tower_http::cors::{Any, CorsLayer};


#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(Any);

    let config = aos_dispatcher::config::Config::new();
    let server = SharedState::new(config).await;

    // build our application with a single route
    let app = Router::new()
        .route("/ping", get(|| async { "pong" }))
        .route("/sign", post(sign))
        .route("/register_worker", post(register_worker))
        .route("/receive_heart_beat", post(receive_heart_beat))
        .route("/api/question", post(tee_question_handler))
        .route("/api/tee_callback", post(tee_callback))
        .route("/api/opml_question", post(opml_question_handler))
        .route("/api/opml_callback", post(opml_callback))
        .route("/api/list_models", post(list_models))
        .route("/admin/list_workers", post(list_workers))
        .route("/admin/list_questions", post(list_questions_handler))
        .route("/admin/list_answers", post(list_answers_handler))
        .layer(cors)
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(handle_error))
                .timeout(Duration::from_secs(600))
                .layer(TraceLayer::new_for_http())
        )
        .with_state(server);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

