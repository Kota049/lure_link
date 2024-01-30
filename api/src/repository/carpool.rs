use crate::domain::domain_object::id::Id;
use crate::entity::recruitment::CarPool;
use crate::error::Error;
use axum::async_trait;

#[async_trait]
pub trait CarPoolRepositoryTrait {
    async fn find_all(&self) -> Result<Vec<CarPool>, Error>;
    async fn create(&self, car_pool: CarPool) -> Result<CarPool, Error>;
    async fn update(&self, car_pool: CarPool) -> Result<CarPool, Error>;
    async fn delete(&self, carpool_id: &Id) -> Result<(), Error>;
}
