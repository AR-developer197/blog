use axum::{routing::{get, post, put}, Router};
use users::{login, logout, new_access, profile, register};

mod users;

pub fn create_user_routes() -> Router<sqlx::Pool<sqlx::Postgres>>{
    let user_router = Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/logout", post(logout))
        .route("/refresh", put(new_access))
        .route("/profile/{id}", get(profile));

    user_router
}