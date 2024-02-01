use super::*;
use crate::db::connection::DbManager;

#[tokio::test]
async fn test_domain_carpool_status() {
    let db = DbManager::new();
    // get test
    let row = db
        .client()
        .await
        .query_one("SELECT 'APPLYING', 'some'", &[])
        .await
        .unwrap();
    let carpool_status = row.try_get::<_, CarPoolStatus>(0).unwrap();
    assert_eq!(carpool_status, CarPoolStatus::Applying);

    // set test
    let carpool_status = CarPoolStatus::Applying;
    let row = db
        .client()
        .await
        .query_one("SELECT $1::TEXT AS carpool_status", &[&carpool_status])
        .await
        .unwrap();
    let res: CarPoolStatus = row.try_get("carpool_status").unwrap();
    assert_eq!(carpool_status, res);

    // couldn't get invalid prefecture name
    let row = db
        .client()
        .await
        .query_one("SELECT 'invalid'", &[])
        .await
        .unwrap();
    let prefecture = row.try_get::<_, CarPoolStatus>(0);
    assert!(prefecture.is_err());

    // deserialize test
    let str = r#"
    {
        "test": "APPLYING"
    }
    "#;
    #[derive(Deserialize)]
    struct Test {
        pub test: CarPoolStatus,
    }
    let test: Test = serde_json::from_str(str).unwrap();
    assert_eq!(test.test, CarPoolStatus::Applying);

    // couldn't deserialize invalid id
    let str = r#"
    {
        "test":"invalid"
    }
    "#;
    let res = serde_json::from_str::<Test>(str);
    assert!(res.is_err())
}
