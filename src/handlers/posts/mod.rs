mod posts;

use axum::{middleware::from_fn, routing::{delete, get, post, put}, Router};
use posts::{create_post, delete_post, get_post, get_posts, modify_post};
use serde::Deserialize;

use crate::{jwt::Token, middleware::auth};

#[derive(Deserialize, Debug)]
pub struct Post {
    post_id: Option<i32>,
    user_id: Option<i32>,
    title: String,
    body: String,
    publication_date: Option<i32>
}

#[derive(Deserialize, Debug)]
pub struct Body {
    post: Post,
    token: String
}

pub fn create_post_routes() -> Router<sqlx::Pool<sqlx::Postgres>> {
    let routes = Router::new()
        .route("/", get(get_posts))
        .route("/get/{id}", get(get_post))
        .route("/create", post(create_post))
        .route("/modify/{id}", put(modify_post))
        .route("/delete/{id}", delete(delete_post))
        .layer(from_fn(auth));

    routes
}