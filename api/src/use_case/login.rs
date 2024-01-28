use crate::domain::domain_object::application_token::ApplicationToken;
use crate::entity::line::LineToken;
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

        Ok(User {
            id: 1i64.try_into()?,
            application_token: ApplicationToken("".to_string()),
            application_refresh_token: ApplicationToken("".to_string()),
            line_access_token: line_token.access_token,
            line_refresh_token: line_token.refresh_token,
            line_id: line_profile.line_user_id,
        })
    }
    // ユーザーを作成する
    async fn upsert_user(&self, line_id: &String) -> Result<User, Error> {
        todo!()
    }
    // トークンを検証する
    pub async fn verify_token(&self) -> Result<User, Error> {
        todo!()
    }
    // トークンを再取得する
    pub async fn refresh_token(&self, line_token: LineToken) {
        todo!()
    }
}

fn create_token() -> ApplicationToken {
    todo!()
}
