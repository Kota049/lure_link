use crate::domain::domain_object::ja_timestamp::JaTimeStamp;
use crate::domain::domain_object::proposal_status::ProposalStatus;
use crate::entity::proposal::Proposal;
use crate::entity::recruitment::Point;
use crate::entity::users::User;
use crate::error::Error;
use chrono::{Duration, Utc};

#[cfg(test)]
mod tests;

pub fn has_applying(exists_proposal: Result<Proposal, Error>) -> Result<(), Error> {
    match exists_proposal {
        Ok(_) => Err(Error::Other("already applying".to_string())),
        Err(Error::NotFound(_)) => Ok(()),
        Err(e) => Err(e),
    }
}

pub fn is_applicant(user: &User, proposal: &Proposal) -> bool {
    &proposal.user == user
}

pub fn can_cancel_term_by_applicant(now: &JaTimeStamp, proposal: &Proposal) -> bool {
    &proposal.carpool.start_time > now
}

pub fn is_non_participation(p: &Proposal) -> bool {
    matches!(
        p.status,
        ProposalStatus::UserCancel | ProposalStatus::Deny | ProposalStatus::OrganizerCancel
    )
}

pub fn is_including_candidate_pick_up_location(p: &Proposal, pick_up_location: &Point) -> bool {
    let mut pick_up_candidate = Vec::with_capacity(3);
    pick_up_candidate.push(p.hope_pick_up_location_1.clone());
    if let Some(p) = p.hope_pick_up_location_2.clone() {
        pick_up_candidate.push(p);
    }
    if let Some(p) = p.hope_pick_up_location_3.clone() {
        pick_up_candidate.push(p);
    }
    pick_up_candidate.contains(&pick_up_location)
}

pub fn is_acceptable_term(now: &JaTimeStamp, proposal: &Proposal) -> bool {
    let accept_deadline = proposal.carpool.start_time.0.with_timezone(&Utc) - Duration::days(1);
    let now = now.0.with_timezone(&Utc);
    now < accept_deadline
}
