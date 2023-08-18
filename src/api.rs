use std::sync::Arc;

use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use uuid::Uuid;

use crate::{
    types::{CountValue, Counter, NameOrId, Token, TokenPermission},
    AppState,
};

#[axum::debug_handler]
pub async fn write(
    State(state): State<Arc<AppState>>,
    //token: ,
    counter: NameOrId,
    Json(value): Json<CountValue>,
) -> StatusCode {
    //tracing::info!("token: {}", token);
    tracing::info!("counter: {:?}", counter);
    tracing::info!("value: {:?}", value);
    /*let a = state.tokens.lock().unwrap();
    match a.get(&token) {
        None => return StatusCode::FORBIDDEN,
        Some(t) => {
            if !t.permissions.contains(TokenPermission::Write) {
                return StatusCode::FORBIDDEN;
            }
        }
    }

    let mut b = state.counters.lock().unwrap();

    match b.get_mut(&counter) {
        Some(val) => {
            if std::mem::discriminant(&val.value) != std::mem::discriminant(&value) {
                StatusCode::BAD_REQUEST
            }
            else {
                val.value = value;
                StatusCode::OK
            }
        },
        None => StatusCode::NOT_FOUND,
    }*/
    StatusCode::OK
}

pub async fn increment(State(state): State<Arc<AppState>>) -> StatusCode {
    todo!()
}

pub async fn decrement(State(state): State<Arc<AppState>>) -> StatusCode {
    todo!()
}

pub async fn flip(State(state): State<Arc<AppState>>) -> StatusCode {
    todo!()
}

pub async fn increment_read(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Counter>, StatusCode> {
    todo!()
}

pub async fn decrement_read(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Counter>, StatusCode> {
    todo!()
}

pub async fn flip_read(State(state): State<Arc<AppState>>) -> Result<Json<Counter>, StatusCode> {
    todo!()
}

pub async fn read(State(state): State<Arc<AppState>>) -> Result<Json<Counter>, StatusCode> {
    todo!()
}

pub async fn get_counters(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<Counter>>, StatusCode> {
    todo!()
}

pub async fn new_counter(State(state): State<Arc<AppState>>) -> Result<Json<Counter>, StatusCode> {
    todo!()
}

pub async fn delete_counter(State(state): State<Arc<AppState>>) -> StatusCode {
    todo!()
}

pub async fn new_token(State(state): State<Arc<AppState>>) -> Result<Json<Token>, StatusCode> {
    todo!()
}
