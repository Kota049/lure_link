use super::*;
use crate::db::connection::connection;
use crate::db::connection::tests::connection_hoge;

#[tokio::test]
async fn get_recruitments() {
    let client = connection().await.unwrap();

    let result = get_recruitment_summary_list(&client).await;

    assert!(result.is_ok());
}
#[tokio::test]
async fn db_error() {
    let client = connection_hoge().await.unwrap();

    let result = get_recruitment_summary_list(&client).await;

    assert_eq!(result, Err(DB_ERROR.to_string()));
}
