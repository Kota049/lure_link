use super::*;
use crate::db::connection::connection;
use tokio_postgres::NoTls;

#[tokio::test]
async fn get_recruitments() {
    let client = connection().await.unwrap();

    let result = get_recruitment_summary_list(&client).await;

    assert!(result.is_ok());
}
#[tokio::test]
async fn db_error() {
    let (client, _) = tokio_postgres::connect("", NoTls).await.unwrap();

    let result = get_recruitment_summary_list(&client).await;

    assert_eq!(result, Err("データが取得できませんでした".to_string()));
}
