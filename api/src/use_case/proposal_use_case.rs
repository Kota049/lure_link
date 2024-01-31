use crate::domain::domain_object::id::Id;
use crate::entity::proposal::{Proposal, UpdateProposal};
use crate::entity::users::User;
use crate::error::Error;
use crate::repository::carpool::CarPoolRepositoryTrait;
use crate::repository::proposal::ProposalRepositoryTrait;
use crate::use_case::proposal_use_case::dto::AplProposal;
use std::sync::Arc;

pub struct ProposalUseCase {
    pr: Arc<dyn ProposalRepositoryTrait + Send + Sync>,
    cpr: Arc<dyn CarPoolRepositoryTrait + Send + Sync>,
}

#[cfg(test)]
pub mod tests;

pub mod dto;

impl ProposalUseCase {
    pub async fn create(&self, applicant: User, input: AplProposal) -> Result<Proposal, Error> {
        todo!()
    }
    pub async fn cancel_by_applicant(
        &self,
        applicant: User,
        proposal_id: Id,
    ) -> Result<Proposal, Error> {
        todo!()
    }
    pub async fn accept_proposal(
        &self,
        organizer: User,
        proposal_id: Id,
    ) -> Result<Proposal, Error> {
        todo!()
    }
    pub async fn edit_proposal(
        &self,
        user: User,
        input: UpdateProposal,
    ) -> Result<Proposal, Error> {
        todo!()
    }
}
