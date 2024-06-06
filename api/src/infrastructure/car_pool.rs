use axum::async_trait;
use crate::domain::domain_object::carpool_status::CarPoolStatus;
use crate::domain::domain_object::id::Id;
use crate::entity::car_pool::{CarPool, CreateCarPool};
use crate::entity::user::User;
use crate::error::Error;
use crate::repository::carpool::CarPoolRepositoryTrait;

pub struct CarPoolRepository;

impl CarPoolRepository {
    pub fn new() -> Result<Self, Error> {
        Ok(Self)
    }
}

#[async_trait]
impl CarPoolRepositoryTrait for CarPoolRepository {
    async fn find_all(&self) -> Result<Vec<CarPool>, Error> {
        Ok(Vec::new())
    }
    async fn find_by_id(&self, id: &Id) -> Result<CarPool, Error> {
        Ok(CarPool {
            id: 1.try_into().unwrap(),
            organizer: Default::default(),
            start_time: Default::default(),
            end_time: Default::default(),
            apl_deadline: Default::default(),
            departure: Default::default(),
            destination: Default::default(),
            budget: 100.try_into().unwrap(),
            max_participant: 0,
            current_participant: 0,
            status: CarPoolStatus::Applying,
            description: "".to_string(),
        })
    }
    async fn create(&self, car_pool: CreateCarPool, user: &User) -> Result<CarPool, Error> {
        Err(Error::Other("not implemented".to_string()))
    }
    async fn update(&self, car_pool: CarPool) -> Result<CarPool, Error> {
        Err(Error::Other("not implemented".to_string()))
    }
    async fn delete(&self, carpool_id: &Id) -> Result<(), Error> {
        Err(Error::Other("not implemented".to_string()))
    }
}