use axum::{debug_handler, Extension};
use axum::extract::{Path, State};
use crate::domain::domain_object::id::Id;
use crate::entity::user::User;
use crate::error::Error;
use crate::infrastructure::AppState;
use crate::use_case::proposal_use_case::dto::PaymentInfo;
use crate::use_case::proposal_use_case::ProposalUseCase;

#[debug_handler]
pub async fn get_payment(Extension(user): Extension<User>, State(app_state): State<AppState>, Path(car_pool_id): Path<Id>) -> Result<PaymentInfo, Error> {
    let uc = ProposalUseCase::new(
        app_state.proposal_repository.clone(),
        app_state.car_pool_repository.clone(),
        app_state.stripe_repository.clone(),
        app_state.user_repository.clone(),
    );
    uc.get_payment(user, car_pool_id).await
}