use axum::{body::{to_bytes, Bytes}, extract::{FromRequest, Request}, middleware::Next, response::{IntoResponse, Response}, Json};
use axum_extra::extract::CookieJar;
use serde::Serialize;

use crate::{error::{ErrorMessage, HttpError}, jwt::Token};

// pub async fn auth(mut req: Request, next: Next) -> Result<Response, HttpError> {
//     let bytes = to_bytes(req.into_body().c, usize::MAX).await
//         .map_err(|e| HttpError::server_error(e.to_string()))?;

//     let body = String::from_utf8(bytes.to_vec())
//         .map_err(|e| HttpError::server_error(e.to_string()))?;

//     let token: Token = serde_json::from_str(&body)
//         .map_err(|e| HttpError::server_error(e.to_string()))?;

//     println!("{:#?}", token);

//     next.run(req).await;

//     Ok(().into_response())
// }

pub async fn auth(jar: CookieJar, mut req: Request, next: Next) -> Result<Response, HttpError> {

    for cookie in jar.iter() {
        println!("Received cookie: {}", cookie.value());
    }

    let cookie = jar.get("refresh_token")
        .map(|cookie| cookie.value().to_owned());

    let token = cookie
        .ok_or_else(|| HttpError::unauthorized(ErrorMessage::SessionCookieMissing.to_string()))?;

    println!("ye0");

    let claims = (Token{token}).validate_token("refresh_token")
        .map_err(|e| HttpError::unauthorized(e.to_string()))?;

    next.run(req).await;

    Ok(Json(claims).into_response())
}