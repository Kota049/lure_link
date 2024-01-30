use crate::domain::domain_object::carpool_status::CarPoolStatus;
use crate::domain::domain_object::id::Id;
use crate::entity::recruitment::{CarPool, CreateCarPool};
use crate::entity::users::User;
use crate::error::Error;
use crate::repository::carpool::CarPoolRepositoryTrait;
use crate::use_case::carpool_use_case::dto::CancelCarPool;
use crate::use_case::carpool_use_case::CarPoolUseCase;
use axum::async_trait;
use mockall::automock;
use std::sync::Arc;

#[automock]
pub trait CarPoolValue {
    fn find_all(&self) -> Result<Vec<CarPool>, Error>;
    fn find_by(&self) -> Result<CarPool, Error>;
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

    async fn find_by_id(&self, _id: &Id) -> Result<CarPool, Error> {
        self.inner.find_by()
    }

    async fn create(&self, _car_pool: CreateCarPool, _u: &User) -> Result<CarPool, Error> {
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
    let input = CancelCarPool::default();
    let user = User::default();
    // キャンセルに成功した場合はCarPoolを返す
    let mut cr = MockCarPoolValue::new();
    cr.expect_update().returning(|| Ok(CarPool::default()));
    cr.expect_find_by().returning(|| Ok(CarPool::default()));
    let uc = CarPoolUseCase {
        cr: Arc::new(MockCarPoolRepo { inner: cr }),
    };
    let res = uc.cancel_carpool(input.clone(), user.clone()).await;
    assert!(res.is_ok());

    // リソースが見つからない場合はエラーを返す
    let mut cr = MockCarPoolValue::new();
    cr.expect_update().returning(|| Ok(CarPool::default()));
    cr.expect_find_by()
        .returning(|| Err(Error::NotFound("not found".to_string())));
    let uc = CarPoolUseCase {
        cr: Arc::new(MockCarPoolRepo { inner: cr }),
    };
    let res = uc.cancel_carpool(input.clone(), user.clone()).await;
    assert!(res.is_err());

    // 更新に失敗した場合にはエラーを返す
    let mut cr = MockCarPoolValue::new();
    cr.expect_update()
        .returning(|| Err(Error::DbError("error occurs".to_string())));
    cr.expect_find_by().returning(|| Ok(CarPool::default()));
    let uc = CarPoolUseCase {
        cr: Arc::new(MockCarPoolRepo { inner: cr }),
    };
    let res = uc.cancel_carpool(input.clone(), user.clone()).await;
    assert!(res.is_err());

    // すでにキャンセルされている場合にはエラーを返す
    let mut cr = MockCarPoolValue::new();
    cr.expect_update().returning(|| Ok(CarPool::default()));
    cr.expect_find_by().returning(|| {
        Ok(CarPool {
            status: CarPoolStatus::Cancel,
            ..CarPool::default()
        })
    });
    let uc = CarPoolUseCase {
        cr: Arc::new(MockCarPoolRepo { inner: cr }),
    };
    let res = uc.cancel_carpool(input.clone(), user.clone()).await;
    assert!(res.is_err());

    // ユーザーと主催者が違う場合はエラーを返す
    let user = User {
        id: 42i64.try_into().unwrap(),
        ..User::default()
    };
    let mut cr = MockCarPoolValue::new();
    cr.expect_update().returning(|| Ok(CarPool::default()));
    cr.expect_find_by().returning(|| Ok(CarPool::default()));
    let uc = CarPoolUseCase {
        cr: Arc::new(MockCarPoolRepo { inner: cr }),
    };
    let res = uc.cancel_carpool(input.clone(), user.clone()).await;
    assert!(res.is_err());
}

#[tokio::test]
async fn test_find_car_pool() {
    todo!()
}
