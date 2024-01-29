use crate::domain::domain_object::application_token::ApplicationToken;
use crate::entity::line::{LineProfile, LineToken};
use crate::entity::users::User;
use crate::error::Error;
use crate::use_case::user_use_case::dto::UpdateUser;
use axum::async_trait;

#[async_trait]
pub trait UserRepositoryTrait {
    // find user_use_case by application token
    async fn find_by_application_token(
        &self,
        application_token: &ApplicationToken,
    ) -> Result<User, Error>;
    // create user_use_case
    async fn create(
        &self,
        line_token: &LineToken,
        line_profile: &LineProfile,
    ) -> Result<User, Error>;
    // find user_use_case by line id
    async fn find_by_line_id(&self, line_id: &String) -> Result<User, Error>;

    // update application token
    async fn update_token(&self, refresh_token: &ApplicationToken) -> Result<User, Error>;

    // update user
    async fn register_user(&self, up: UpdateUser) -> Result<User, Error>;
}
