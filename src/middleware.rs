use axum::{extract::Request, middleware::Next, response::Response};
use axum_extra::extract::CookieJar;

use crate::{error::{ErrorMessage, HttpError}, jwt::Token};

pub async fn auth(jar: CookieJar, mut req: Request, next: Next) -> Result<Response, HttpError> {

    let cookies = jar
        .get("refresh_token")
        .map(|cookie| cookie.value().to_string());

    let token = cookies
        .ok_or_else(|| HttpError::unauthorized(ErrorMessage::SessionCookieMissing.to_string()))?;

    let claims = (Token{token}).validate_token("refresh_secret")
        .map_err(|e| HttpError::unauthorized(e.to_string()))?;

    req.extensions_mut().insert(claims);

    Ok(next.run(req).await)
}