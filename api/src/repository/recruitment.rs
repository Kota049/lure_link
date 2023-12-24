use axum::async_trait;
use crate::entity::recruitment::Recruitment;
use crate::error::Error;

#[async_trait]
pub trait RecruitmentTrait{
    async fn get_all(&self)->Result<Recruitment,Error>;
}