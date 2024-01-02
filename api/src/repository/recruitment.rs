use axum::async_trait;
use chrono::{DateTime, Utc};
use crate::entity::recruitment::Recruitment;
use crate::error::Error;

#[async_trait]
pub trait RecruitmentTrait {
    async fn find_all(&self, now: &DateTime<Utc>) -> Result<Vec<Recruitment>, Error>;
}