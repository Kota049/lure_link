use super::*;
use crate::db::connection::DbManager;

#[tokio::test]
async fn test_domain_proposal_status() {
    let db = DbManager::new();
    // get test
    let row = db
        .client()
        .await
        .query_one("SELECT 'APPLYING', 'some'", &[])
        .await
        .unwrap();
    let proposal_status = row.try_get::<_, ProposalStatus>(0).unwrap();
    assert_eq!(proposal_status, ProposalStatus::Applying);

    // set test
    let proposal_status = ProposalStatus::Applying;
    let row = db
        .client()
        .await
        .query_one("SELECT $1::TEXT AS proposal_status", &[&proposal_status])
        .await
        .unwrap();
    let res: ProposalStatus = row.try_get("proposal_status").unwrap();
    assert_eq!(proposal_status, res);

    // couldn't get invalid prefecture name
    let row = db
        .client()
        .await
        .query_one("SELECT 'invalid'", &[])
        .await
        .unwrap();
    let prefecture = row.try_get::<_, ProposalStatus>(0);
    assert!(prefecture.is_err());

    // deserialize test
    let str = r#"
    {
        "test": "APPLYING"
    }
    "#;
    #[derive(Deserialize)]
    struct Test {
        pub test: ProposalStatus,
    }
    let test: Test = serde_json::from_str(str).unwrap();
    assert_eq!(test.test, ProposalStatus::Applying);

    // couldn't deserialize invalid id
    let str = r#"
    {
        "test":"invalid"
    }
    "#;
    let res = serde_json::from_str::<Test>(str);
    assert!(res.is_err())
}
