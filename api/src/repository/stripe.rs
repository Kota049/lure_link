use axum::async_trait;
use stripe::PaymentIntent;
use crate::entity::car_pool::CarPool;
use crate::entity::user::User;
use crate::error::Error;

#[async_trait]
pub trait StripeRepositoryTrait {
    async fn create_stripe_user(&self, u: User) -> Result<User, Error>;
    async fn get_ephemeral_key(&self, u: User) -> Result<String, Error>;
    async fn create_payment_intent(&self, u: User, c: CarPool) -> Result<PaymentIntent, Error>;
}