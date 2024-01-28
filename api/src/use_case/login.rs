use crate::domain::domain_object::application_token::ApplicationToken;
use crate::entity::line::{LineProfile, LineToken};
use crate::entity::users::User;
use crate::error::Error;
use crate::repository::line::LineClientTrait;
use crate::repository::user::UserRepositoryTrait;
use std::sync::Arc;

#[cfg(test)]
mod tests;

pub struct LoginUseCase {
    u_repo: Arc<dyn UserRepositoryTrait + Send + Sync>,
    line_client: Arc<dyn LineClientTrait + Send + Sync>,
}

impl LoginUseCase {
    // Lineの確認コードからデータを取得→正常ならトークンを発行、なければトークンを削除
    pub async fn verify_user(&self, _code: String) -> Result<User, Error> {
        // get token
        let line_token = self.line_client.get_token(&_code).await?;
        let line_profile = self.line_client.get_profile(line_token.clone()).await?;
        self.find_or_create_user(line_token, line_profile).await
    }
    // ユーザーを作成する
    async fn find_or_create_user(
        &self,
        line_token: LineToken,
        line_profile: LineProfile,
    ) -> Result<User, Error> {
        let exists_user = self
            .u_repo
            .find_by_line_id(&line_profile.line_user_id)
            .await;
        if let Ok(u) = exists_user {
            return Ok(u);
        }
        if let Err(Error::NotFound(_)) = exists_user {
            return self.u_repo.create(&line_token, &line_profile).await;
        }
        exists_user
    }
    // トークンを検証する
    pub async fn verify_token(&self, application_token: &ApplicationToken) -> Result<User, Error> {
        self.u_repo
            .find_by_application_token(application_token)
            .await
    }
    // トークンを再取得する
    pub async fn refresh_token(&self, refresh_token: &ApplicationToken) -> Result<User, Error> {
        todo!()
    }
}
