use super::*;
use crate::db::connection::DbManager;

#[tokio::test]
async fn test_domain_payment_status() {
    let db = DbManager::new();
    // get test
    let row = db
        .client()
        .await
        .query_one("SELECT 'AUTHORIZATION', 'some'", &[])
        .await
        .unwrap();
    let payment_status = row.try_get::<_, PaymentStatus>(0).unwrap();
    assert_eq!(payment_status, PaymentStatus::Authorization);

    // set test
    let payment_status = PaymentStatus::Authorization;
    let row = db
        .client()
        .await
        .query_one("SELECT $1::TEXT AS payment_status", &[&payment_status])
        .await
        .unwrap();
    let res: PaymentStatus = row.try_get("payment_status").unwrap();
    assert_eq!(payment_status, res);

    // couldn't get invalid prefecture name
    let row = db
        .client()
        .await
        .query_one("SELECT 'invalid'", &[])
        .await
        .unwrap();
    let prefecture = row.try_get::<_, PaymentStatus>(0);
    assert!(prefecture.is_err());

    // deserialize test
    let str = r#"
    {
        "test": "AUTHORIZATION"
    }
    "#;
    #[derive(Deserialize)]
    struct Test {
        pub test: PaymentStatus,
    }
    let test: Test = serde_json::from_str(str).unwrap();
    assert_eq!(test.test, PaymentStatus::Authorization);

    // couldn't deserialize invalid id
    let str = r#"
    {
        "test":"invalid"
    }
    "#;
    let res = serde_json::from_str::<Test>(str);
    assert!(res.is_err())
}
