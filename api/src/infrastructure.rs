use std::sync::Arc;
use crate::error::Error;
use crate::infrastructure::car_pool::CarPoolRepository;
use crate::infrastructure::proposal::ProposalRepository;
use crate::infrastructure::stripe::StripeRepository;
use crate::infrastructure::user::UserRepository;
use crate::repository::carpool::CarPoolRepositoryTrait;
use crate::repository::proposal::ProposalRepositoryTrait;
use crate::repository::stripe::StripeRepositoryTrait;
use crate::repository::user::UserRepositoryTrait;

pub mod stripe;
pub mod user;
pub mod proposal;
pub mod car_pool;

#[derive(Clone)]
pub struct AppState {
    pub stripe_repository: Arc<dyn StripeRepositoryTrait + Send + Sync>,
    pub user_repository: Arc<dyn UserRepositoryTrait + Send + Sync>,
    pub proposal_repository: Arc<dyn ProposalRepositoryTrait + Send + Sync>,
    pub car_pool_repository: Arc<dyn CarPoolRepositoryTrait + Send + Sync>,
}

impl AppState {
    pub fn init() -> Result<AppState, Error> {
        let stripe_repository = StripeRepository::new()?;
        let user_repository = UserRepository::new()?;
        let proposal_repository = ProposalRepository::new()?;
        let carpool_repository = CarPoolRepository::new()?;

        Ok(AppState {
            stripe_repository: Arc::new(stripe_repository),
            user_repository: Arc::new(user_repository),
            proposal_repository: Arc::new(proposal_repository),
            car_pool_repository: Arc::new(carpool_repository),
        })
    }
}
