use axum::async_trait;
use crate::entity::line::{LineProfile, LineToken};
use crate::error::Error;

#[async_trait]
pub trait LineClientTrait {
    // lineAPIからトークン情報を取得する
    async fn get_token(&self, code: &str) -> Result<LineToken, Error>;
    // lineAPIからprofile情報を取得する
    async fn get_profile(&self, line_token: LineToken) -> Result<LineProfile, Error>;
}