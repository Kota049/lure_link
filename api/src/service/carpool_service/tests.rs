use crate::domain::domain_object::carpool_status::CarPoolStatus;
use crate::entity::recruitment::CarPool;
use crate::service::carpool_service::{is_canceled, modify_to_cancel};

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
    let res_other = CarPool {
        status: CarPoolStatus::Cancel,
        ..res
    };
    assert_eq!(res_other, c)
}
