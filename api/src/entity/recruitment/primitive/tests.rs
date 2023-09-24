use super::super::primitive::Recruitment;
use super::*;
use crate::domain::budget::Budget;
use crate::domain::id::ID;
use crate::domain::municipality::Municipality;
use crate::domain::nick_name::NickName;
use crate::domain::participant_count::ParticipantCount;
use crate::domain::point_name::PointName;
use crate::domain::prefecture::Prefecture;
use crate::domain::start_time::StartDate;
use crate::domain::Domain;
#[test]
fn valid() {
    let primitive_recruitment = PrimitiveRecruitment {
        id: "1".to_string(),
        organizer_nick_name: "サンプル太郎".to_string(),
        start_date: "2023-09-24 10:00:00".to_string(),
        rendezvous_prefecture: "千葉県".to_string(),
        rendezvous_municipality: "富津市".to_string(),
        rendezvous_point: "富津岬".to_string(),
        destination_prefecture: "埼玉県".to_string(),
        destination_municipality: "川越市".to_string(),
        destination_point: "川越駅前".to_string(),
        budget: "1000".to_string(),
        participant_count: "3".to_string(),
    };
    let result = primitive_recruitment.sophisticate();
    let expected = Ok(Recruitment {
        id: ID::new("1").unwrap(),
        organizer_nick_name: NickName::new("サンプル太郎").unwrap(),
        start_date: StartDate::new("2023-09-24 10:00:00").unwrap(),
        rendezvous_prefecture: Prefecture::new("千葉県").unwrap(),
        rendezvous_municipality: Municipality::new("富津市").unwrap(),
        rendezvous_point: PointName::new("富津岬").unwrap(),
        destination_prefecture: Prefecture::new("埼玉県").unwrap(),
        destination_municipality: Municipality::new("川越市").unwrap(),
        destination_point: PointName::new("川越駅前").unwrap(),
        budget: Budget::new("1000").unwrap(),
        participant_count: ParticipantCount::new("3").unwrap(),
    });
    assert_eq!(result, expected);
}
