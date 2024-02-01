use crate::db::connection::DbManager;
use super::*;

#[tokio::test]
async fn test_domain_object_email() {
    let db = DbManager::new();
    // get test
    let row = db.client().await.query_one("SELECT 'aaa@example.com', 'some'", &[]).await.unwrap();
    let name = row.try_get::<_, MailAddress>(0).unwrap();
    assert_eq!(name, MailAddress("aaa@example.com".to_string()));

    // set test
    let name = MailAddress("aaa@example.com".to_string());
    let row = db.client().await.query_one("SELECT $1::TEXT AS name", &[&name]).await.unwrap();
    let res: MailAddress = row.try_get("name").unwrap();
    assert_eq!(name, res);

    // couldn't get invalid name
    let row = db.client().await.query_one("SELECT ''", &[]).await.unwrap();
    let mail = row.try_get::<_, MailAddress>(0);
    println!("mail = {mail:?}");
    assert!(mail.is_err());

    // deserialize test
    let str = r#"
    {
        "test": "aaa@example.com"
    }
    "#;
    #[derive(Deserialize)]
    struct Test {
        pub test: MailAddress,
    }
    let test: Test = serde_json::from_str(str).unwrap();
    assert_eq!(test.test, MailAddress("aaa@example.com".to_string()));

    // couldn't deserialize invalid id
    let str = r#"
    {
        "test":""
    }
    "#;
    let res = serde_json::from_str::<Test>(str);
    assert!(res.is_err())
}
