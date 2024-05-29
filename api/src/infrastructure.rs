use std::sync::Arc;
use crate::error::Error;
use crate::infrastructure::stripe::StripeRepository;
use crate::infrastructure::user::UserRepository;
use crate::repository::stripe::StripeRepositoryTrait;
use crate::repository::user::UserRepositoryTrait;

pub mod stripe;
pub mod user;

pub struct AppState {
    pub stripe_repository: Arc<dyn StripeRepositoryTrait + Send + Sync>,
    pub user_repository: Arc<dyn UserRepositoryTrait + Send + Sync>,
}

impl AppState {
    pub fn init() -> Result<AppState, Error> {
        let stripe_repository = StripeRepository::new()?;
        let user_repository = UserRepository::new()?;
        Ok(AppState {
            stripe_repository: Arc::new(stripe_repository),
            user_repository: Arc::new(user_repository),
        })
    }
}
