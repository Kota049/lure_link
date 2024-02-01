use crate::domain::domain_object::ja_timestamp::JaTimeStamp;
use crate::domain::domain_object::proposal_status::ProposalStatus;
use crate::entity::proposal::Proposal;
use crate::entity::users::User;
use crate::error::Error;

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
