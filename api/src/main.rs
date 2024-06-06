use axum::{routing::get, Router, middleware};
use axum::routing::{put};
use tokio;
use api::infrastructure::AppState;
use api::presentation::handlers::proposal::get_payment;
use api::presentation::middleware::auth;

#[tokio::main]
async fn main() {
    let port_number = std::env::var("RUST_PORT").unwrap_or_else(|e| {
        eprint!("{e}");
        String::from("4000")
    });
    let bind_target = format!("0.0.0.0:{}", port_number);
    let app_state = AppState::init().unwrap();
    let non_auth_router = Router::new().route("/hello", get(|| async { "Hello world" }));
    let auth_router = Router::new()
        .route("/carpool/:id/payment-info", put(get_payment))
        .route_layer(middleware::from_fn_with_state(app_state.clone(), auth));

    let app =
        Router::new()
            .merge(non_auth_router)
            .merge(auth_router)
            .with_state(app_state);

    let listener = tokio::net::TcpListener::bind(bind_target).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}