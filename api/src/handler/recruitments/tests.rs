use super::*;
use crate::db::app_state::ClientState;
use crate::db::connection::connection;
use crate::db::connection::tests::connection_hoge;
use axum::body::HttpBody;
use axum::extract::State;
use axum::http::StatusCode;
use serde_json::Value;
use std::sync::Arc;

#[tokio::test]
async fn valid_response() {
    let client = connection().await.unwrap();
    let state = Arc::new(ClientState { client });

    let mut result = read_recruitments(State(state)).await.unwrap();

    assert_eq!(StatusCode::OK, result.status());
    let bytes = result.data().await.unwrap().unwrap();
    let body = bytes.as_ref();
    let body_str = String::from_utf8(Vec::from(body)).unwrap();
    let value: Value = serde_json::from_str(&body_str).unwrap();
    let expected = value.as_array().unwrap();

    assert!(expected.iter().all(|v| { v.get("id").is_some() }));
    assert!(expected
        .iter()
        .all(|v| { v.get("organizer_nick_name").is_some() }));
    assert!(expected.iter().all(|v| { v.get("start_date").is_some() }));
    assert!(expected
        .iter()
        .all(|v| { v.get("rendezvous_prefecture").is_some() }));
    assert!(expected
        .iter()
        .all(|v| { v.get("rendezvous_municipality").is_some() }));
    assert!(expected
        .iter()
        .all(|v| { v.get("rendezvous_point").is_some() }));
    assert!(expected
        .iter()
        .all(|v| { v.get("destination_prefecture").is_some() }));
    assert!(expected
        .iter()
        .all(|v| { v.get("destination_municipality").is_some() }));
    assert!(expected
        .iter()
        .all(|v| { v.get("destination_point").is_some() }));
    assert!(expected.iter().all(|v| { v.get("budget").is_some() }));
    assert!(expected
        .iter()
        .all(|v| { v.get("participant_count").is_some() }));
}

#[tokio::test]
async fn db_error() {
    let client = connection_hoge().await.unwrap();
    let state = Arc::new(ClientState { client });

    let mut result = read_recruitments(State(state)).await.err().unwrap();

    assert_eq!(StatusCode::OK, result.status());
    let body = result.data().await.unwrap().unwrap();
    assert_eq!(body, "データが取得できませんでした");
}
