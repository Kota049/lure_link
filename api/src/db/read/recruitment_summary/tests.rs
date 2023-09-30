use super::*;
use crate::db::connection::connection;

#[tokio::test]
async fn get_recruitments() {
    let client = connection().await.unwrap();

    let result = get_recruitment_summary_list(&client).await;

    assert!(result.is_ok());
}
