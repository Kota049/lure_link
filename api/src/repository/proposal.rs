use crate::domain::domain_object::id::Id;
use crate::domain::domain_object::proposal_status::ProposalStatus;
use crate::entity::proposal::{CreateProposal, Proposal, UpdateProposal};
use crate::entity::recruitment::CarPool;
use crate::entity::users::User;
use crate::error::Error;
use axum::async_trait;

#[async_trait]
pub trait ProposalRepositoryTrait {
    async fn find(&self, id: &Id) -> Result<Proposal, Error>;
    async fn find_by_user_and_carpool(
        &self,
        user_id: &User,
        car_pool: &CarPool,
    ) -> Result<Proposal, Error>;
    async fn find_by_carpool_id(&self, carpool_id: &Id) -> Result<Vec<Proposal>, Error>;
    async fn create(&self, user: &User, create: CreateProposal) -> Result<Proposal, Error>;
    async fn update(&self, user: &User, update_proposal: UpdateProposal)
        -> Result<Proposal, Error>;
    async fn update_proposal_status(
        &self,
        proposal_id: Id,
        status: ProposalStatus,
    ) -> Result<Proposal, Error>;
    async fn delete(&self, id: &Id) -> Result<(), Error>;
}
