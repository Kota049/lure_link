use std::sync::Arc;
use axum::{routing::get, Router, middleware};
use axum::http::Request;
use axum::middleware::Next;
use tokio;
use api::infrastructure::AppState;
use api::presentation::middleware::auth;

#[tokio::main]
async fn main() {
    let port_number = std::env::var("RUST_PORT").unwrap_or_else(|e| {
        eprint!("{e}");
        String::from("4000")
    });
    let bind_target = format!("0.0.0.0:{}", port_number);

    let app_state = Arc::new(AppState::init().unwrap());

    let unguard_route = Router::new().route("/", get(|| async { "Hello, World!" }));
    let guard_route = Router::new()
        .route("/a", get(|| async { "HELLO WORLD" }))
        .layer(middleware::from_fn(auth));

    let app = Router::new()
        .nest("/", unguard_route)
        .nest("/", guard_route)
        .layer(axum::middleware::from_fn(move |mut req: Request<_>, next: Next<_>| {
            let state = Arc::clone(&app_state);
            async move {
                req.extensions_mut().insert(state);
                next.run(req).await
            }
        }));

    axum::Server::bind(&bind_target.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
