use super::*;
use crate::db::connection::DbManager;

#[tokio::test]
async fn test_domain_user_status() {
    let db = DbManager::new();
    // get test
    let row = db
        .client()
        .await
        .query_one("SELECT 'TRIAL', 'some'", &[])
        .await
        .unwrap();
    let user_status = row.try_get::<_, UserStatus>(0).unwrap();
    assert_eq!(user_status, UserStatus::Trial);

    // set test
    let user_status = UserStatus::Trial;
    let row = db
        .client()
        .await
        .query_one("SELECT $1::TEXT AS user_status", &[&user_status])
        .await
        .unwrap();
    let res: UserStatus = row.try_get("user_status").unwrap();
    assert_eq!(user_status, res);

    // couldn't get invalid prefecture name
    let row = db
        .client()
        .await
        .query_one("SELECT 'invalid'", &[])
        .await
        .unwrap();
    let prefecture = row.try_get::<_, UserStatus>(0);
    assert!(prefecture.is_err());

    // deserialize test
    let str = r#"
    {
        "test": "TRIAL"
    }
    "#;
    #[derive(Deserialize)]
    struct Test {
        pub test: UserStatus,
    }
    let test: Test = serde_json::from_str(str).unwrap();
    assert_eq!(test.test, UserStatus::Trial);

    // couldn't deserialize invalid id
    let str = r#"
    {
        "test":"invalid"
    }
    "#;
    let res = serde_json::from_str::<Test>(str);
    assert!(res.is_err())
}
