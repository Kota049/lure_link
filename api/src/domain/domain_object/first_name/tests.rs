use super::*;
use crate::db::connection::DbManager;

#[tokio::test]
async fn test_domain_object_first_name() {
    let db = DbManager::new();
    // get test
    let row = db
        .client()
        .await
        .query_one("SELECT '田中', 'some'", &[])
        .await
        .unwrap();
    let name = row.try_get::<_, FirstName>(0).unwrap();
    assert_eq!(name, FirstName("田中".to_string()));

    // set test
    let name = FirstName("田中".to_string());
    let row = db
        .client()
        .await
        .query_one("SELECT $1::TEXT AS name", &[&name])
        .await
        .unwrap();
    let res: FirstName = row.try_get("name").unwrap();
    assert_eq!(name, res);

    // couldn't get invalid name
    let row = db.client().await.query_one("SELECT ''", &[]).await.unwrap();
    let prefecture = row.try_get::<_, FirstName>(0);
    assert!(prefecture.is_err());

    // deserialize test
    let str = r#"
    {
        "test": "田中"
    }
    "#;
    #[derive(Deserialize)]
    struct Test {
        pub test: FirstName,
    }
    let test: Test = serde_json::from_str(str).unwrap();
    assert_eq!(test.test, FirstName("田中".to_string()));

    // couldn't deserialize invalid id
    let str = r#"
    {
        "test":""
    }
    "#;
    let res = serde_json::from_str::<Test>(str);
    assert!(res.is_err())
}
