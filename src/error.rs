use axum::response::{IntoResponse, Response};
use hyper::StatusCode;

pub struct HttpError {
    status: StatusCode,
    message: String 
}

impl std::fmt::Display for HttpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.status, self.message)
    }
}

impl HttpError {
    pub fn server_error<E>(error: E) -> Self
    where
        E: std::error::Error + Send + Sync
    {
        Self {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: error.to_string()
        }
    }

    pub fn into_response(self) -> Response{
        (self.status, self.status).into_response()
    }
}

impl IntoResponse for HttpError {
    fn into_response(self) -> axum::response::Response {
        self.into_response()
    }
}