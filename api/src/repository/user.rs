use axum::async_trait;
use crate::entity::line::{LineProfile, LineToken};
use crate::entity::users::User;
use crate::error::Error;

#[async_trait]
pub trait UserRepositoryTrait {
    // アクセストークンによるユーザーの取得
    async fn find_by_application_token(&self, token: &String) -> Result<User, Error>;
    // ユーザーの作成
    async fn create(&self, line_token: &LineToken, line_profile: &LineProfile) -> Result<User, Error>;
}