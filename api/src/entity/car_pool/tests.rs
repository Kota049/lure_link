use super::*;
use crate::domain::Domain;
use serde_json::json;

#[test]
fn to_value() {
    let recruitment = Recruitment {
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
    };
    let result = recruitment.to_value();
    let expected = json!({
        "id":"1",
        "organizer_nick_name":"サンプル太郎",
        "start_date":"2023-09-24 10:00:00",
        "rendezvous_prefecture":"千葉県",
        "rendezvous_municipality":"富津市",
        "rendezvous_point":"富津岬",
        "destination_prefecture":"埼玉県",
        "destination_municipality":"川越市",
        "destination_point":"川越駅前",
        "budget":"1000",
        "participant_count":"3",
    });
    assert_eq!(result, expected)
}
