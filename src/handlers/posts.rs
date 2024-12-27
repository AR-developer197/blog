use axum::extract::Path;

pub async fn posts() -> &'static str {
    "posts"
}

pub async fn post() -> &'static str {
    "post"
}

pub async fn create_post() -> &'static str {
    "create_post"
}

pub async fn modify_post() -> &'static str {
    "modify_post"
}

pub async fn delete_post() -> &'static str {
    "delete_post"
}