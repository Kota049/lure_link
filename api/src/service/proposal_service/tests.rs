use crate::domain::domain_object::ja_timestamp::JaTimeStamp;
use crate::domain::domain_object::proposal_status::ProposalStatus;
use crate::entity::car_pool::{CarPool, Point};
use crate::entity::proposal::Proposal;
use crate::entity::user::User;
use crate::error::Error;
use crate::service::proposal_service::{
    can_cancel_term_by_applicant, has_applying, is_acceptable_term, is_applicant,
    is_including_candidate_pick_up_location, is_non_participation,
};
use chrono::{Duration, Utc};

#[test]
fn test_has_applying() {
    let exists = Ok(Proposal::default());
    let res = has_applying(exists);
    assert!(res.is_err());

    let exists = Err(Error::Other("".to_string()));
    let res = has_applying(exists);
    assert!(res.is_err());

    let exists = Err(Error::NotFound("".to_string()));
    let res = has_applying(exists);
    assert!(res.is_ok());
}

#[test]
fn test_is_applicant() {
    let user = User::default();
    let proposal = Proposal::default();
    let res = is_applicant(&user, &proposal);
    assert!(res);

    let user = User {
        id: 42i64.try_into().unwrap(),
        ..User::default()
    };
    let res = is_applicant(&user, &proposal);
    assert!(!res);
}

#[test]
fn test_can_cancel_term_by_applicant() {
    let now = Utc::now();
    let after = (now + Duration::days(1)).try_into().unwrap();
    let before = (now - Duration::days(1)).try_into().unwrap();
    let now: JaTimeStamp = now.try_into().unwrap();

    let res = can_cancel_term_by_applicant(
        &now,
        &Proposal {
            carpool: CarPool {
                start_time: after,
                ..CarPool::default()
            },
            ..Proposal::default()
        },
    );
    assert!(res);
    let res = can_cancel_term_by_applicant(
        &now,
        &Proposal {
            carpool: CarPool {
                start_time: before,
                ..CarPool::default()
            },
            ..Proposal::default()
        },
    );
    assert!(!res);
}

#[test]
fn test_is_non_participation() {
    let proposal = Proposal {
        status: ProposalStatus::UserCancel,
        ..Proposal::default()
    };
    let res = is_non_participation(&proposal);
    assert!(res);

    let proposal = Proposal {
        status: ProposalStatus::OrganizerCancel,
        ..Proposal::default()
    };
    let res = is_non_participation(&proposal);
    assert!(res);

    let proposal = Proposal {
        status: ProposalStatus::Deny,
        ..Proposal::default()
    };
    let res = is_non_participation(&proposal);
    assert!(res);

    let proposal = Proposal {
        status: ProposalStatus::Applying,
        ..Proposal::default()
    };
    let res = is_non_participation(&proposal);
    assert!(!res);
}

#[test]
fn test_is_including_candidate_pick_up_location() {
    let pick_up = Point::default();
    let another_pick_up = Point {
        municipality: "invalid".to_string().try_into().unwrap(),
        ..Point::default()
    };

    let proposal = Proposal {
        hope_pick_up_location_1: another_pick_up.clone(),
        ..Proposal::default()
    };
    let res = is_including_candidate_pick_up_location(&proposal, &pick_up);
    assert!(!res);

    let proposal = Proposal {
        hope_pick_up_location_1: pick_up.clone(),
        ..Proposal::default()
    };
    let res = is_including_candidate_pick_up_location(&proposal, &pick_up);
    assert!(res);

    let proposal = Proposal {
        hope_pick_up_location_1: another_pick_up.clone(),
        hope_pick_up_location_2: Some(pick_up.clone()),
        ..Proposal::default()
    };
    let res = is_including_candidate_pick_up_location(&proposal, &pick_up);
    assert!(res);
}

#[test]
fn test_is_acceptable_term() {
    let now = Utc::now();
    let expired = (now + Duration::hours(12)).try_into().unwrap();
    let valid = (now + Duration::days(2)).try_into().unwrap();

    let valid_proposal = Proposal {
        carpool: CarPool {
            start_time: valid,
            ..CarPool::default()
        },
        ..Proposal::default()
    };
    let res = is_acceptable_term(&now.try_into().unwrap(), &valid_proposal);
    assert!(res);

    let expired_proposal = Proposal {
        carpool: CarPool {
            start_time: expired,
            ..CarPool::default()
        },
        ..Proposal::default()
    };
    let res = is_acceptable_term(&now.try_into().unwrap(), &expired_proposal);
    assert!(!res);
}
