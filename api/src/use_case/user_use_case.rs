use crate::domain::domain_object::application_token::ApplicationToken;
use crate::entity::line::{LineProfile, LineToken};
use crate::entity::user::User;
use crate::error::Error;
use crate::repository::line::LineClientTrait;
use crate::repository::user::UserRepositoryTrait;
use crate::service::validation::register_user::validate_first_register_user;
use crate::use_case::user_use_case::dto::UpdateUser;
use std::sync::Arc;

#[cfg(test)]
pub mod tests;

pub mod dto;

pub struct UserUseCase {
    u_repo: Arc<dyn UserRepositoryTrait + Send + Sync>,
    line_client: Arc<dyn LineClientTrait + Send + Sync>,
}

impl UserUseCase {
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
        self.u_repo.update_token(refresh_token).await
    }
    // 登録する
    pub async fn first_register_user(&self, user: UpdateUser) -> Result<User, Error> {
        self.verify_token(&user.application_token).await?;
        validate_first_register_user(&user)?;
        self.u_repo.register_user(user).await
    }
}
