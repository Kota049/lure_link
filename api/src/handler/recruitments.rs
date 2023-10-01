use crate::db::app_state::ClientState;
use crate::db::read::recruitment_summary::get_recruitment_summary_list;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::{json, Value};
use std::sync::Arc;

#[cfg(test)]
mod tests;

pub async fn read_recruitments(
    State(state): State<Arc<ClientState>>,
) -> Result<Response, Response> {
    let primitive_recruitments = get_recruitment_summary_list(&state.client).await.unwrap();

    let mut result: Vec<Value> = Vec::new();
    for recruitment in primitive_recruitments {
        let recruitment = recruitment.sophisticate();
        if recruitment.is_err() {
            continue;
        }
        let recruitment = recruitment.unwrap();
        result.push(recruitment.to_value());
    }
    Ok((StatusCode::OK, json!(result).to_string()).into_response())
}
