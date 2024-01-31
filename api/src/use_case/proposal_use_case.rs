use crate::domain::domain_object::id::Id;
use crate::domain::domain_object::ja_timestamp::JaTimeStamp;
use crate::domain::domain_object::proposal_status::ProposalStatus;
use crate::entity::proposal::{CreateProposal, Proposal, UpdateProposal};
use crate::entity::users::User;
use crate::error::Error;
use crate::error::Error::{AuthenticateError, Other};
use crate::repository::carpool::CarPoolRepositoryTrait;
use crate::repository::proposal::ProposalRepositoryTrait;
use crate::service::carpool_service::is_organizer;
use crate::service::{carpool_service, proposal_service};
use crate::use_case::proposal_use_case::dto::AplProposal;
use chrono::Utc;
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
        let now: JaTimeStamp = Utc::now()
            .try_into()
            .map_err(|_| Error::Other("internal server error".to_string()))?;
        let carpool = self.cpr.find_by_id(&input.car_pool_id).await?;
        if is_organizer(&carpool, &applicant) {
            return Err(Error::Other("you are organizer".to_string()));
        }
        let exists_proposal = self.pr.find_by_user_and_carpool(&applicant, &carpool).await;
        proposal_service::has_applying(exists_proposal)?;

        if carpool_service::can_apl_term(&now, &carpool) {
            return Err(Error::Other("expired applying deadline".to_string()));
        }

        let create_proposal = CreateProposal {
            carpool,
            hope_pick_up_location_1: input.hope_pick_up_location_1.clone(),
            hope_pick_up_location_2: input.hope_pick_up_location_2.clone(),
            hope_pick_up_location_3: input.hope_pick_up_location_3.clone(),
            payment_status: input.payment_status.clone(),
            apl_time: now,
            description: input.description.clone(),
        };
        let proposal = self.pr.create(&applicant, create_proposal).await?;

        Ok(proposal)
    }
    pub async fn cancel_by_applicant(
        &self,
        applicant: User,
        proposal_id: Id,
    ) -> Result<Proposal, Error> {
        let now: JaTimeStamp = Utc::now()
            .try_into()
            .map_err(|_| Error::Other("internal server error".to_string()))?;
        let proposal = self.pr.find(&proposal_id).await?;
        if proposal.user != applicant {
            return Err(AuthenticateError("you are not applicant".to_string()));
        }
        if proposal.carpool.start_time < now {
            return Err(Other("expired cancel deadline".to_string()));
        }
        Ok(proposal)
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
