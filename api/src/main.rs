use api::db::app_state::ClientState;
use api::db::connection::connection;
use api::handler::recruitments::read_recruitments;
use axum::{routing::get, Router};
use std::sync::Arc;
use tokio;

#[tokio::main]
async fn main() {
    let client = connection().await.unwrap();
    let state = Arc::new(ClientState { client });
    let port_number = std::env::var("RUST_PORT").unwrap_or_else(|e| {
        eprint!("{e}");
        String::from("4000")
    });
    let bind_target = format!("0.0.0.0:{}", port_number);
    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/api/v1/recruitments", get(read_recruitments))
        .with_state(state);
    axum::Server::bind(&bind_target.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
