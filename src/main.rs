use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::Duration,
};

use axum::{
    routing::{delete, get, post},
    Router,
};
use tower_http::trace::TraceLayer;
use types::{Counter, Token};
use uuid::Uuid;
mod api;
mod types;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    tracing::info!("Starting counters...");

    match run().await {
        Ok(_) => tracing::info!("Program exited successfully."),
        Err(e) => tracing::error!("Error: {}", e),
    }
}

async fn run() -> anyhow::Result<()> {
    tracing::info!("Database initialized.");
    let state = Arc::new(AppState {
        counters: Mutex::new(HashMap::new()),
        tokens: Mutex::new(HashMap::new()),
    });

    let app = Router::new()
        .route("/", get(|| async { "YES SIR OORAH" }))
        .route("/api/v1/:counter/write", post(api::write))
        /*.route("/api/v1/:counter/increment", post(api::increment))
        .route("/api/v1/:counter/decrement", post(api::decrement))
        .route("/api/v1/:counter/flip", post(api::flip))
        .route("/api/v1/:counter/increment_read", post(api::increment_read))
        .route("/api/v1/:counter/decrement_read", post(api::decrement_read))
        .route("/api/v1/:counter/flip_read", post(api::flip_read))
        .route("/api/v1/:counter/read", get(api::read))
        .route("/api/v1/get_counters", get(api::get_counters))
        .route("/api/v1/new_counter", post(api::new_counter))
        .route("/api/v1/delete_counter", delete(api::delete_counter))
        .route("/api/v1/new_token", post(api::new_token))
        */
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let listener = std::net::TcpListener::bind("0.0.0.0:3000")?;
    tracing::info!("Listening on {}...", listener.local_addr()?);

    axum::Server::from_tcp(listener)?
        .serve(app.into_make_service())
        .await?;
    Ok(())
}

pub struct AppState {
    counters: Mutex<HashMap<i64, Counter>>,
    tokens: Mutex<HashMap<Uuid, Token>>,
}
