use std::sync::Arc;
use crate::entity::users::User;
use crate::error::Error;
use crate::repository::line::LineClientTrait;
use crate::repository::user::UserRepositoryTrait;

#[cfg(test)]
mod tests;

pub struct LoginUseCase {
    u_repo: Arc<dyn UserRepositoryTrait + Send + Sync>,
    line_client: Arc<dyn LineClientTrait + Send + Sync>,
}

impl LoginUseCase {
    // Lineの確認コードからデータを取得→正常ならトークンを発行、なければトークンを削除
    pub async fn verify_user(&self, _code: String) -> Result<User, Error> {
        Ok(User {
            id: 1i64.try_into()?,
            application_token: "".to_string(),
            application_refresh_token: "".to_string(),
            line_access_token: "".to_string(),
            line_refresh_token: "".to_string(),
            line_id: "".to_string(),
        })
    }
    // ユーザーを作成する
    async fn create_user(&self) {}
    // トークンを検証する
    pub async fn verify_token(&self) {}
    // トークンを再取得する
    pub async fn refresh_token(&self) {}
    async fn create_token(&self) {}
}