use axum::{extract::{FromRequestParts, Path, rejection::PathRejection}, async_trait, http::{StatusCode, request::Parts}};
use bitflags::bitflags;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub struct Token {
    pub token: Uuid,
    pub permissions: TokenPermission,
    pub lifespan: TokenLifespan,
}

bitflags! {
    #[derive(Debug, Clone, Copy, Eq, PartialEq)]
    pub struct TokenPermission: u32 {
        const Manage = 0b00000001;
        const Read = 0b00000010;
        const Write = 0b00000100;
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum TokenLifespan {
    Once,
    Fixed(u32),
    Indefinite,
}

#[derive(Debug, Clone)]
pub struct Counter {
    pub id: i64,
    pub name: String,
    pub value: CountValue,
}

#[derive(Debug, Clone)]
pub enum NameOrId {
    Id(i64),
    Name(String),
}

#[async_trait]
impl<S> FromRequestParts<S> for NameOrId
where
    S: Send + Sync,
{
    type Rejection = PathRejection;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let Path(counter) = Path::<String>::from_request_parts(parts, state).await?;

        if let Ok(id) = counter.parse::<i64>() {
            Ok(NameOrId::Id(id))
        } else {
            Ok(NameOrId::Name(counter.clone()))
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum CountValue {
    Signed(i32),
    Unsigned(u32),
    Boolean(bool),
}

pub trait Inc<T> {
    fn increment(&mut self) {}
}

pub trait Dec<T> {
    fn decrement(&mut self) {}
}

pub trait Flip<T> {
    fn flip(&mut self) {}
}
