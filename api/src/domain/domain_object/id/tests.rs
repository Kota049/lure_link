use serde::Deserialize;
use crate::db::connection::DbManager;
use crate::domain::domain_object::id::Id;

#[tokio::test]
async fn test_domain_object_id() {
    let db = DbManager::new();
    // get test
    let row = db.client().await.query_one("SELECT 100::BIGINT, 'some'", &[]).await.unwrap();
    let id: Id = row.try_get(0).unwrap();
    assert_eq!(id, Id(100));

    // set test
    let id = Id(42);
    let row = db.client().await.query_one("SELECT $1::TEXT::BIGINT AS id", &[&id]).await.unwrap();
    let id: Id = row.try_get("id").unwrap();
    assert_eq!(id, Id(42));

    // couldn't get negative id
    let row = db.client().await.query_one("SELECT -42::BIGINT", &[]).await.unwrap();
    let id = row.try_get::<_, Id>(0);
    assert!(id.is_err());

    // deserialize test
    let str = r#"
    {
        "id":42
    }
    "#;
    #[derive(Deserialize)]
    struct Test {
        pub id: Id,
    }
    let id: Test = serde_json::from_str(str).unwrap();
    assert_eq!(id.id, Id(42));

    // couldn't deserialize invalid id
    let str = r#"
    {
        "id":-42
    }
    "#;
    let id = serde_json::from_str::<Test>(str);
    assert!(id.is_err())
}
