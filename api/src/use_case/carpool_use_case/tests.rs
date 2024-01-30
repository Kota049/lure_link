use crate::domain::domain_object::id::Id;
use crate::entity::recruitment::{CarPool, CreateCarPool};
use crate::entity::users::User;
use crate::error::Error;
use crate::repository::carpool::CarPoolRepositoryTrait;
use crate::use_case::carpool_use_case::CarPoolUseCase;
use axum::async_trait;
use mockall::automock;
use std::sync::Arc;

#[automock]
pub trait CarPoolValue {
    fn find_all(&self) -> Result<Vec<CarPool>, Error>;
    fn create(&self) -> Result<CarPool, Error>;
    fn update(&self) -> Result<CarPool, Error>;
    fn delete(&self) -> Result<(), Error>;
}

struct MockCarPoolRepo {
    inner: MockCarPoolValue,
}

#[async_trait]
impl CarPoolRepositoryTrait for MockCarPoolRepo {
    async fn find_all(&self) -> Result<Vec<CarPool>, Error> {
        self.inner.find_all()
    }
    async fn create(&self, _car_pool: CarPool) -> Result<CarPool, Error> {
        self.inner.create()
    }
    async fn update(&self, _car_pool: CarPool) -> Result<CarPool, Error> {
        self.inner.update()
    }
    async fn delete(&self, _carpool_id: &Id) -> Result<(), Error> {
        self.inner.delete()
    }
}

#[tokio::test]
async fn test_create_carpool() {
    let input = CreateCarPool::default();
    let user = User::default();
    // 正常な場合はCarpoolを返す
    let mut cr = MockCarPoolValue::new();
    cr.expect_create().returning(|| Ok(CarPool::default()));
    let uc = CarPoolUseCase {
        cr: Arc::new(MockCarPoolRepo { inner: cr }),
    };
    let res = uc.create_carpool(input.clone(), user.clone()).await;
    assert!(res.is_ok());

    // 作成に失敗した場合はエラーを返す
    let mut cr = MockCarPoolValue::new();
    cr.expect_create()
        .returning(|| Err(Error::DbError("error occurs".to_string())));
    let uc = CarPoolUseCase {
        cr: Arc::new(MockCarPoolRepo { inner: cr }),
    };
    let res = uc.create_carpool(input.clone(), user.clone()).await;
    assert!(res.is_err())
}

#[tokio::test]
async fn test_cancel_carpool() {
    todo!()
}

#[tokio::test]
async fn test_find_car_pool() {
    todo!()
}
