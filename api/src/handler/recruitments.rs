use crate::db::app_state::ClientState;
use axum::extract::State;
use axum::response::Response;
use std::sync::Arc;

#[cfg(test)]
mod tests;

pub async fn read_recruitments(
    State(state): State<Arc<ClientState>>,
) -> Result<Response, Response> {
    todo!()
}
