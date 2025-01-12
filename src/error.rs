use axum::response::IntoResponse;
use hyper::StatusCode;

pub struct HttpError {
    status: StatusCode,
    message: String 
}

pub enum ErrorMessage {
    WrongPassword,
    EmptyPassword
}

impl ToString for ErrorMessage {
    fn to_string(&self) -> String {
        self.to_str().to_owned()
    }
}

impl ErrorMessage {
    fn to_str(&self) -> &'static str{
        match self {
            ErrorMessage::WrongPassword => "Wrong Password",
            ErrorMessage::EmptyPassword => "Empty Field"
        }
    }
}

impl std::error::Error for HttpError {}

impl std::fmt::Display for HttpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.status, self.message)
    }
}

impl std::fmt::Debug for HttpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.status, self.message)
    }
}

impl HttpError {
    pub fn new(error: impl Into<String>, status: StatusCode) -> HttpError {
        Self {
            status,
            message: error.into()
        }
    }

    pub fn server_error(error: impl Into<String>) -> Self {
        Self {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: error.into()
        }
    }

    pub fn unauthorized(error: impl Into<String>) -> Self {
        Self {
            status: StatusCode::UNAUTHORIZED,
            message: error.into()
        }
    }

    pub fn forbidden(error: impl Into<String>) -> Self {
        Self {
            status: StatusCode::FORBIDDEN,
            message: error.into()
        }
    }

    pub fn unique_violation(error: impl Into<String>) -> Self {      
        Self {
            status: StatusCode::CONFLICT,
            message: error.into()
        }
    }

    pub fn into_error_response(self) -> (StatusCode, String) {
        (self.status, self.message.to_string())
    }
}

impl IntoResponse for HttpError {
    fn into_response(self) -> axum::response::Response {
        self.into_error_response().into_response()
    }
}