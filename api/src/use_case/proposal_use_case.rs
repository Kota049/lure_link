use crate::domain::domain_object::carpool_status::CarPoolStatus;
use crate::domain::domain_object::id::Id;
use crate::domain::domain_object::ja_timestamp::JaTimeStamp;
use crate::domain::domain_object::proposal_status::ProposalStatus;
use crate::entity::proposal::{AcceptProposal, CreateProposal, Proposal, UpdateProposal};
use crate::entity::recruitment::CarPool;
use crate::entity::users::User;
use crate::error::Error;
use crate::error::Error::{AuthenticateError, Other};
use crate::repository::carpool::CarPoolRepositoryTrait;
use crate::repository::proposal::ProposalRepositoryTrait;
use crate::service::carpool_service::is_organizer;
use crate::service::proposal_service::is_non_participation;
use crate::service::time_service::get_ja_now;
use crate::service::{carpool_service, proposal_service, time_service};
use crate::use_case::proposal_use_case::dto::AplProposal;
use chrono::{Duration, Utc};
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
        let now = time_service::get_ja_now()?;
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
        let now = time_service::get_ja_now()?;
        let proposal = self.pr.find(&proposal_id).await?;
        if !proposal_service::is_applicant(&applicant, &proposal) {
            return Err(AuthenticateError("you are not applicant".to_string()));
        }
        if !proposal_service::can_cancel_term_by_applicant(&now, &proposal) {
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
        if proposal.carpool.status == CarPoolStatus::AplComplete {
            return Err(Other("already accepted".to_string()));
        }
        let mut pick_up_candidate = Vec::with_capacity(3);
        pick_up_candidate.push(proposal.hope_pick_up_location_1);
        if let Some(p) = proposal.hope_pick_up_location_2.clone() {
            pick_up_candidate.push(p);
        }
        if let Some(p) = proposal.hope_pick_up_location_3.clone() {
            pick_up_candidate.push(p);
        }
        if !pick_up_candidate.contains(&accept_proposal.pick_up_point) {
            return Err(Other("invalid pick up point".to_string()));
        }

        let accept_deadline: JaTimeStamp = (proposal.carpool.start_time.0.with_timezone(&Utc)
            - Duration::days(1))
        .try_into()
        .unwrap();
        if accept_deadline < now {
            return Err(Other("expired acceptable term".to_string()));
        }

        let update_carpool = CarPool {
            current_participant: proposal.carpool.current_participant.clone() + 1,
            ..proposal.carpool.clone()
        };
        self.cpr.update(update_carpool).await?;
        self.pr.accept(accept_proposal).await
    }
    pub async fn edit_proposal(
        &self,
        user: User,
        input: UpdateProposal,
    ) -> Result<Proposal, Error> {
        todo!()
    }
}
