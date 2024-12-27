use std::net::SocketAddr;

use axum::{routing::{delete, get, post, put}, Router};
use handlers::{comments, create_comments, create_post, delete_comments, delete_post, login, modify_post, posts, profile, register};
use tokio::net::TcpListener;

mod handlers;

#[tokio::main]
async fn main() -> std::io::Result<()> {

    let user_routes = Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/profile/:id", get(profile));

    let posts_routes = Router::new()
        .route("/", get(posts))
        .route("/get/:id", get(handlers::post))
        .route("/create", post(create_post))
        .route("/modify/:id", put(modify_post))
        .route("/delete/:id", delete(delete_post));

    let comments_routes = Router::new()
        .route("/", get(comments))
        .route("/create", post(create_comments))
        .route("/delete", delete(delete_comments));

    let app = Router::new()
        .route("/", get(|| async {"hello blog"}))
        .nest("/users", user_routes)
        .nest("/posts", posts_routes)
        .nest("/comments", comments_routes);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    let listener = TcpListener::bind(addr).await?;

    axum::serve(listener, app).await?;

    Ok(())
}
