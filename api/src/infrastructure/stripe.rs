use std::str::FromStr;
use axum::async_trait;
use stripe::{Client, CreateCustomer, CreateEphemeralKey, CreatePaymentIntent, Currency, Customer, CustomerId, EphemeralKey, PaymentIntent, PaymentIntentCaptureMethod};
use crate::entity::proposal::Proposal;
use crate::entity::user::User;
use crate::error::Error;
use crate::error::Error::Other;
use crate::repository::stripe::StripeRepositoryTrait;

// WARNING:テストしてないよ
// WARNING:多分Stripeの方でテスト環境と本番環境が別れてるからテストできるはず。
// WARNING:とりあえず仮実装
pub struct StripeRepository {
    client: Client,
}

impl StripeRepository {
    pub fn new() -> Result<Self, Error> {
        let secret_key = dotenv::var("STRIPE_SECRET_KEY").map_err(|e| Error::Other(e.to_string()))?;
        Ok(Self {
            client: Client::new(secret_key)
        })
    }
}

#[async_trait]
impl StripeRepositoryTrait for StripeRepository {
    async fn create_stripe_user(&self, u: User) -> Result<User, Error> {
        // init CreateCustomer
        let create_customer = CreateCustomer::new();
        let response = Customer::create(&self.client, create_customer).await;
        if let Err(e) = response {
            return Err(Error::Other(e.to_string()));
        }
        let customer_id = response.unwrap().id;
        Ok(User {
            stripe_user_id: Some(customer_id.to_string()),
            ..u
        })
    }

    async fn get_ephemeral_key(&self, u: User) -> Result<String, Error> {
        if u.stripe_user_id.is_none() {
            return Err(Error::Other("user doesn't have customer id in Stripe".to_string()));
        }
        let customer_id_string = &u.stripe_user_id.unwrap();
        let customer_id = CustomerId::from_str(customer_id_string).map_err(|e| {
            Error::Other(e.to_string())
        })?;
        let create_epheramel_key = CreateEphemeralKey {
            customer: Some(customer_id),
            expand: &[],
            issuing_card: None,
            nonce: None,
            verification_session: None,
        };
        let epheramel_key = EphemeralKey::create(&self.client, create_epheramel_key).await.map_err(|e| Error::Other(e.to_string()))?;
        epheramel_key.secret.ok_or(Error::Other("cannot create epheramel key".to_string()))
    }

    async fn create_payment_intent(&self, u: User, p: Proposal) -> Result<PaymentIntent, Error> {
        // FIXME parse carpool's budget to i64
        let mut create_payment = CreatePaymentIntent::new(1000, Currency::JPY);
        // あとでキャプチャする
        create_payment.capture_method = Some(PaymentIntentCaptureMethod::Manual);
        // 顧客情報
        if let Some(customer_id) = u.stripe_user_id {
            create_payment.customer = CustomerId::from_str(&customer_id).ok();
        }
        PaymentIntent::create(&self.client, create_payment).await.map_err(|e| Other(e.to_string()))
    }
}