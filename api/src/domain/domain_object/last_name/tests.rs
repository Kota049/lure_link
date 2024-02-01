use super::*;
use crate::db::connection::DbManager;

#[tokio::test]
async fn test_domain_object_last_name() {
    let db = DbManager::new();
    // get test
    let row = db
        .client()
        .await
        .query_one("SELECT '太郎', 'some'", &[])
        .await
        .unwrap();
    let name = row.try_get::<_, LastName>(0).unwrap();
    assert_eq!(name, LastName("太郎".to_string()));

    // set test
    let name = LastName("太郎".to_string());
    let row = db
        .client()
        .await
        .query_one("SELECT $1::TEXT AS name", &[&name])
        .await
        .unwrap();
    let res: LastName = row.try_get("name").unwrap();
    assert_eq!(name, res);

    // couldn't get invalid name
    let row = db.client().await.query_one("SELECT ''", &[]).await.unwrap();
    let prefecture = row.try_get::<_, LastName>(0);
    assert!(prefecture.is_err());

    // deserialize test
    let str = r#"
    {
        "test": "太郎"
    }
    "#;
    #[derive(Deserialize)]
    struct Test {
        pub test: LastName,
    }
    let test: Test = serde_json::from_str(str).unwrap();
    assert_eq!(test.test, LastName("太郎".to_string()));

    // couldn't deserialize invalid id
    let str = r#"
    {
        "test":""
    }
    "#;
    let res = serde_json::from_str::<Test>(str);
    assert!(res.is_err())
}
