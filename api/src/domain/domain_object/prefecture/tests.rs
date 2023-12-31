use crate::db::connection::DbManager;
use super::*;

#[tokio::test]
async fn test_domain_object_prefecture() {
    let db = DbManager::new();
    // get test
    let row = db.client().await.query_one("SELECT '埼玉県', 'some'", &[]).await.unwrap();
    let prefecture = row.try_get::<_, Prefecture>(0).unwrap();
    assert_eq!(prefecture, Prefecture::Saitama);

    // set test
    let prefecture = Prefecture::Saitama;
    let row = db.client().await.query_one("SELECT $1::TEXT AS prefecture", &[&prefecture]).await.unwrap();
    let res: Prefecture = row.try_get("prefecture").unwrap();
    assert_eq!(prefecture, res);

    // couldn't get invalid prefecture name
    let row = db.client().await.query_one("SELECT 'invalid'", &[]).await.unwrap();
    let prefecture = row.try_get::<_, Prefecture>(0);
    assert!(prefecture.is_err());

    // deserialize test
    let str = r#"
    {
        "test": "埼玉県"
    }
    "#;
    #[derive(Deserialize)]
    struct Test {
        pub test: Prefecture,
    }
    let test: Test = serde_json::from_str(str).unwrap();
    assert_eq!(test.test, Prefecture::Saitama);

    // couldn't deserialize invalid id
    let str = r#"
    {
        "test":"invalid"
    }
    "#;
    let res = serde_json::from_str::<Test>(str);
    assert!(res.is_err())
}
