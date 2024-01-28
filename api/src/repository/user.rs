use crate::domain::domain_object::application_token::ApplicationToken;
use crate::entity::line::{LineProfile, LineToken};
use crate::entity::users::User;
use crate::error::Error;
use axum::async_trait;

#[async_trait]
pub trait UserRepositoryTrait {
    // アクセストークンによるユーザーの取得
    async fn find_by_application_token(
        &self,
        application_token: &ApplicationToken,
    ) -> Result<User, Error>;
    // ユーザーの作成
    async fn create(
        &self,
        line_token: &LineToken,
        line_profile: &LineProfile,
    ) -> Result<User, Error>;
    // line tokenによるユーザーの取得
    async fn find_by_line_token(&self, line_id: &String) -> Result<User, Error>;
}
