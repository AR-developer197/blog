use std::future::Future;
use axum::{extract::{FromRef, FromRequestParts}, http::request::Parts};
use hyper::StatusCode;
use sqlx::postgres::PgPool;

pub struct DatabaseConnection(pub sqlx::pool::PoolConnection<sqlx::Postgres>);

impl<S> FromRequestParts<S> for DatabaseConnection
where
    PgPool: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    fn from_request_parts(_parts: &mut Parts, state: &S,) 
        -> impl Future<Output = Result<Self, Self::Rejection>> + Send
    {
        Box::pin(async move {
            let pool = PgPool::from_ref(state);
            
            let conn = pool.acquire().await.map_err(internal_error)?;

            Ok(DatabaseConnection(conn))
        })
    }
}

pub fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error + Send + Sync,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}