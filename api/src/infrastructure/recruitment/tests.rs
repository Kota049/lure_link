use chrono::Utc;
use chrono_tz::Asia::Tokyo;
use crate::infrastructure::recruitment::RecruitmentRepository;
use crate::repository::recruitment::RecruitmentTrait;

#[tokio::test]
async fn test_find_all(){
    let now = Utc::now();
    let repo = RecruitmentRepository::new();
    let res = repo.find_all(&now).await;
    assert!(res.is_ok());
    // filter only now < apl_deadline
    let recruitments = res.unwrap();
    assert!(recruitments.iter().all(|r|r.apl_deadline.0 > Utc::now().with_timezone(&Tokyo)));
}