use axum::async_trait;
use crate::domain::domain_object::application_token::ApplicationToken;
use crate::entity::line::{LineProfile, LineToken};
use crate::entity::user::User;
use crate::error::Error;
use crate::repository::user::UserRepositoryTrait;
use crate::use_case::user_use_case::dto::UpdateUser;

pub struct UserRepository;

impl UserRepository {
    pub fn new() -> Result<Self, Error> {
        Ok(Self)
    }
}

// todo 実装
#[async_trait]
impl UserRepositoryTrait for UserRepository {
    async fn find_by_application_token(&self, application_token: &ApplicationToken) -> Result<User, Error> {
        Ok(User::default())
    }
    async fn create(&self, line_token: &LineToken, line_profile: &LineProfile) -> Result<User, Error> {
        Ok(User::default())
    }
    async fn find_by_line_id(&self, line_id: &String) -> Result<User, Error> {
        Ok(User::default())
    }
    async fn update_token(&self, refresh_token: &ApplicationToken) -> Result<User, Error> {
        Ok(User::default())
    }
    async fn register_user(&self, up: UpdateUser) -> Result<User, Error> {
        Ok(User::default())
    }

    async fn save(&self, user: User) -> Result<User, Error> {
        Ok(user)
    }
}