use chrono::{Datelike, Timelike, TimeZone};
use crate::db::connection::DbManager;
use super::*;

#[tokio::test]
async fn test_domain_object_ja_timestamp() {
    let utc_time:DateTime<Utc> = Utc.with_ymd_and_hms(2015, 1, 1, 0, 0, 0).unwrap();
    let db = DbManager::new();
    // get test
    let row = db.client().await.query_one("SELECT '2015-01-01 09:00:00'::timestamp AT TIME ZONE 'Asia/Tokyo', 'some'", &[]).await.unwrap();
    let ja_time = row.try_get::<_, JaTimeStamp>(0).unwrap();
    assert_eq!(ja_time, JaTimeStamp(utc_time.with_timezone(&Tokyo)));

    // set test
    let ja_time = JaTimeStamp(utc_time.with_timezone(&Tokyo));
    let row = db.client().await.query_one("SELECT $1::text::timestamp with time zone AS ja_time", &[&ja_time]).await.unwrap();
    let res: JaTimeStamp = row.try_get("ja_time").unwrap();
    assert_eq!(res.0.day(),1u32);
    assert_eq!(res.0.year(),2015i32);
    assert_eq!(res.0.month(),1u32);
    assert_eq!(res.0.hour(),9u32);
    assert_eq!(res.0.minute(),0u32);
    assert_eq!(res.0.second(),0u32);



    // couldn't get invalid name
    let row = db.client().await.query_one("SELECT ''", &[]).await.unwrap();
    let ja_time = row.try_get::<_, JaTimeStamp>(0);
    assert!(ja_time.is_err());

    // deserialize test
    let str = r#"
    {
        "test": "2015-01-01 00:00:00+00:00"
    }
    "#;
    #[derive(Deserialize)]
    struct Test {
        pub test: JaTimeStamp,
    }
    let test: Test = serde_json::from_str(str).unwrap();
    assert_eq!(test.test, JaTimeStamp(utc_time.with_timezone(&Tokyo)));

    // couldn't deserialize invalid id
    let str = r#"
    {
        "test":""
    }
    "#;
    let res = serde_json::from_str::<Test>(str);
    assert!(res.is_err())
}