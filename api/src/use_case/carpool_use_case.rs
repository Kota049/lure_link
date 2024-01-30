use crate::entity::recruitment::{CarPool, CreateCarPool};
use crate::entity::users::User;
use crate::error::Error;
use crate::repository::carpool::CarPoolRepositoryTrait;
use crate::service::carpool_service;
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
        let target_carpool = self.cr.find_by_id(&input.id).await?;
        if &target_carpool.organizer.id != &user.id {
            return Err(Error::AuthenticateError(
                "You are not organizer".to_string(),
            ));
        }
        if carpool_service::is_canceled(&target_carpool) {
            return Err(Error::Other(
                "Cannot cancel carpool because already canceled".to_string(),
            ));
        }
        let cancel_carpool = carpool_service::modify_to_cancel(target_carpool);
        self.cr.update(cancel_carpool).await
    }
    // 募集を全件取得
    pub async fn find_car_pool(&self) -> Result<CarPool, Error> {
        todo!()
    }
}
