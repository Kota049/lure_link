use crate::entity::recruitment::{CarPool, CreateCarPool};
use crate::entity::users::User;
use crate::error::Error;
use crate::repository::carpool::CarPoolRepositoryTrait;
use crate::use_case::carpool_use_case::dto::CancelCarPool;
use std::sync::Arc;

#[cfg(test)]
mod tests;

pub mod dto;

pub struct CarPoolUseCase {
    cr: Arc<dyn CarPoolRepositoryTrait + Send + Sync>,
}

impl CarPoolUseCase {
    // 新規募集の作成
    pub async fn create_carpool(&self, input: CreateCarPool, user: User) -> Result<CarPool, Error> {
        self.cr.create(input, &user).await
    }
    // 募集の削除
    pub async fn cancel_carpool(&self, input: CancelCarPool, user: User) -> Result<CarPool, Error> {
        todo!()
    }
    // 募集を全件取得
    pub async fn find_car_pool(&self) -> Result<CarPool, Error> {
        todo!()
    }
}
