use std::net::SocketAddr;

use axum::{http::HeaderValue, routing::get, Router};
use hyper::{header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE}, Method};
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use sqlx::PgPool;

mod handlers;
mod db;
mod jwt;
mod error;
mod middleware;

use handlers::{
    create_user_routes, create_post_routes
};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let pool = PgPool::connect(&dotenv::var("DB_URL_CONNECTION").unwrap()).await.unwrap();

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE])
        .allow_credentials(true)
        .allow_methods([Method::GET, Method::POST,Method::PUT]);

    let app = Router::new()
        .route("/", get(|| async {"Hello World!"}))
        .nest("/users", create_user_routes())
        .nest("/posts", create_post_routes())
        .with_state(pool)
        .layer(cors);



    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    let listener = TcpListener::bind(addr).await?;

    axum::serve(listener, app).await?;
    
    Ok(())
}
