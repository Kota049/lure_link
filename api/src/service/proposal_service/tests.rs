use crate::domain::domain_object::ja_timestamp::JaTimeStamp;
use crate::domain::domain_object::proposal_status::ProposalStatus;
use crate::entity::proposal::Proposal;
use crate::entity::recruitment::CarPool;
use crate::entity::users::User;
use crate::error::Error;
use crate::service::proposal_service::{
    can_cancel_term_by_applicant, has_applying, is_applicant, is_non_participation,
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
