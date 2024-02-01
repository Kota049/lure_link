use crate::domain::domain_object::id::Id;
use crate::domain::domain_object::proposal_status::ProposalStatus;
use crate::entity::proposal::{AcceptProposal, CreateProposal, Proposal, UpdateProposal};
use crate::entity::user::User;
use crate::error::Error;
use crate::error::Error::{AuthenticateError, Other};
use crate::repository::carpool::CarPoolRepositoryTrait;
use crate::repository::proposal::ProposalRepositoryTrait;
use crate::service::carpool_service::{add_one_accept, is_applying, is_organizer};
use crate::service::proposal_service::{
    can_cancel_term_by_applicant, is_acceptable_term, is_applicant,
    is_including_candidate_pick_up_location, is_non_participation, is_updatable_term_by_applicant,
};
use crate::service::time_service::get_ja_now;
use crate::service::{carpool_service, proposal_service};
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
        let now = get_ja_now()?;
        let carpool = self.cpr.find_by_id(&input.car_pool_id).await?;
        if is_organizer(&carpool, &applicant) {
            return Err(Other("you are organizer".to_string()));
        }
        let exists_proposal = self.pr.find_by_user_and_carpool(&applicant, &carpool).await;
        proposal_service::has_applying(exists_proposal)?;

        if carpool_service::can_apl_term(&now, &carpool) {
            return Err(Other("expired applying deadline".to_string()));
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
        let now = get_ja_now()?;
        let proposal = self.pr.find(&proposal_id).await?;
        if !is_applicant(&applicant, &proposal) {
            return Err(AuthenticateError("you are not applicant".to_string()));
        }
        if !can_cancel_term_by_applicant(&now, &proposal) {
            return Err(Other("expired cancel deadline".to_string()));
        }
        if is_non_participation(&proposal) {
            return Err(Other("already cancel or denied".to_string()));
        }
        let proposal = self
            .pr
            .update_proposal_status(proposal_id, ProposalStatus::UserCancel)
            .await?;
        Ok(proposal)
    }

    pub async fn accept_proposal(
        &self,
        organizer: User,
        accept_proposal: AcceptProposal,
    ) -> Result<Proposal, Error> {
        let now = get_ja_now()?;
        let proposal = self.pr.find(&accept_proposal.id).await?;
        if !is_organizer(&proposal.carpool, &organizer) {
            return Err(AuthenticateError("you are not organizer".to_string()));
        }
        if !is_applying(&proposal.carpool) {
            return Err(Other("already accepted".to_string()));
        }
        if !is_including_candidate_pick_up_location(&proposal, &accept_proposal.pick_up_point) {
            return Err(Other("invalid pick up point".to_string()));
        }
        if !is_acceptable_term(&now, &proposal) {
            return Err(Other("expired acceptable term".to_string()));
        }

        let update_carpool = add_one_accept(proposal.carpool)?;
        self.cpr.update(update_carpool).await?;
        self.pr.accept(accept_proposal).await
    }
    pub async fn edit_proposal(
        &self,
        user: User,
        input: UpdateProposal,
    ) -> Result<Proposal, Error> {
        let proposal = self.pr.find(&input.id).await?;
        if !is_applicant(&user, &proposal) {
            return Err(AuthenticateError("you are not applicant".to_string()));
        }
        if !is_updatable_term_by_applicant(&proposal) {
            return Err(Other("cannot update because already fixed".to_string()));
        }
        self.pr.update(&user, input).await
    }
}
