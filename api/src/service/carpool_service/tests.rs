use crate::domain::domain_object::carpool_status::CarPoolStatus;
use crate::domain::domain_object::ja_timestamp::JaTimeStamp;
use crate::entity::proposal::Proposal;
use crate::entity::recruitment::CarPool;
use crate::entity::users::User;
use crate::service::carpool_service::{
    add_one_accept, can_apl_term, is_applying, is_canceled, is_organizer, modify_to_cancel,
};
use chrono::{Duration, Utc};

#[test]
fn test_is_canceled() {
    let c = CarPool {
        status: CarPoolStatus::Cancel,
        ..CarPool::default()
    };
    let res = is_canceled(&c);
    assert!(res);

    let c = CarPool {
        status: CarPoolStatus::Applying,
        ..CarPool::default()
    };
    let res = is_canceled(&c);
    assert!(!res);
}

#[test]
fn test_modify_to_cancel() {
    let c = CarPool::default();
    let res = modify_to_cancel(c.clone());
    assert_eq!(&res.status, &CarPoolStatus::Cancel);

    // 他の項目が変更されない
    let expected = CarPool {
        status: CarPoolStatus::Cancel,
        ..c
    };
    assert_eq!(res, expected)
}

#[test]
fn test_is_organizer() {
    let c = CarPool::default();
    let u = User::default();
    let res = is_organizer(&c, &u);
    assert!(res);

    let u = User {
        id: 42i64.try_into().unwrap(),
        ..User::default()
    };
    let res = is_organizer(&c, &u);
    assert!(!res);
}

#[test]
fn test_can_apl_term() {
    let now: JaTimeStamp = Utc::now().try_into().unwrap();
    let after = (Utc::now() + Duration::days(1)).try_into().unwrap();
    let c = CarPool {
        apl_deadline: after,
        ..CarPool::default()
    };
    let res = can_apl_term(&now, &c);
    assert!(!res);

    let before = (Utc::now() - Duration::days(1)).try_into().unwrap();
    let c = CarPool {
        apl_deadline: before,
        ..CarPool::default()
    };
    let res = can_apl_term(&now, &c);
    assert!(res);
}

#[test]
fn test_is_applying() {
    let car_pool = CarPool {
        status: CarPoolStatus::Applying,
        ..CarPool::default()
    };
    let res = is_applying(&car_pool);
    assert!(res);

    let car_pool = CarPool {
        status: CarPoolStatus::AplComplete,
        ..CarPool::default()
    };
    let res = is_applying(&car_pool);
    assert!(!res);
}

#[test]
fn test_add_one_accept() {
    // 更新前の現在の参加者がmaxより2人以上少ない時
    let c = CarPool {
        max_participant: 2,
        current_participant: 0,
        ..CarPool::default()
    };
    let res = add_one_accept(c).unwrap();
    assert_eq!(res.current_participant, 1);
    assert_eq!(res.status, CarPoolStatus::Applying);

    // 更新前の現在の参加者がmax-1の場合
    let c = CarPool {
        max_participant: 2,
        current_participant: 1,
        ..CarPool::default()
    };
    let res = add_one_accept(c).unwrap();
    assert_eq!(res.current_participant, 2);
    assert_eq!(res.status, CarPoolStatus::AplComplete);

    // 更新前の現在の参加者がmax以上の場合はエラー
    let c = CarPool {
        max_participant: 2,
        current_participant: 2,
        ..CarPool::default()
    };
    let res = add_one_accept(c);
    assert!(res.is_err());
}
