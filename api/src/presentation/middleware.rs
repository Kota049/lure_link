// use std::sync::Arc;
// use axum::extract::State;
// use axum::http::{Request, StatusCode};
// use axum::middleware::Next;
// use axum::{RequestExt};
// use axum::response::{IntoResponse, Response};
// use tower::ServiceExt;
// use crate::domain::domain_object::application_token::ApplicationToken;
// use crate::error::Error;
// use crate::infrastructure::AppState;
//
// // todo テスト
// pub async fn auth<B>(mut req: Request<B>, next: Next<B>, State(state): State<Arc<AppState>>) -> impl IntoResponse {
//     let header = req.headers().get("Authorization").and_then(|v| v.to_str().ok());
//     if header.is_none() {
//         return StatusCode::UNAUTHORIZED.into_response();
//     }
//     let header = header.unwrap();
//
//     if !header.starts_with("Bearer ") {
//         return StatusCode::UNAUTHORIZED.into_response();
//     }
//     let token_str = header[7..].to_string();
//     let token: Result<ApplicationToken, Error> = token_str.try_into();
//     if token.is_err() {
//         return StatusCode::UNAUTHORIZED.into_response();
//     }
//     let token = token.unwrap();
//     let user_repository = Arc::clone(&state.user_repository);
//     if let Ok(user) = user_repository.find_by_application_token(&token).await {
//         req.extensions_mut().insert(user);
//         return next.run(req).await;
//     }
//     StatusCode::UNAUTHORIZED.into_response()
// }

use std::sync::Arc;
use axum::extract::State;
use axum::http::{Request, StatusCode};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use crate::domain::domain_object::application_token::ApplicationToken;
use crate::error::Error;
use crate::infrastructure::AppState;

pub async fn auth<B>(mut req: Request<B>, next: Next<B>) -> impl IntoResponse {
    let state = req.extensions().get::<Arc<AppState>>().cloned().unwrap();

    let header = req.headers().get("Authorization").and_then(|v| v.to_str().ok());
    if header.is_none() {
        return StatusCode::UNAUTHORIZED.into_response();
    }
    let header = header.unwrap();

    if !header.starts_with("Bearer ") {
        return StatusCode::UNAUTHORIZED.into_response();
    }
    let token_str = header[7..].to_string();
    let token: Result<ApplicationToken, Error> = token_str.try_into();
    if token.is_err() {
        return StatusCode::UNAUTHORIZED.into_response();
    }
    let token = token.unwrap();
    let user_repository = Arc::clone(&state.user_repository);
    if let Ok(user) = user_repository.find_by_application_token(&token).await {
        req.extensions_mut().insert(user);
        return next.run(req).await;
    }
    StatusCode::UNAUTHORIZED.into_response()
}
