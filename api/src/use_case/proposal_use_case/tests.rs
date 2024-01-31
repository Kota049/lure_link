use crate::domain::domain_object::id::Id;
use crate::domain::domain_object::ja_timestamp::JaTimeStamp;
use crate::domain::domain_object::proposal_status::ProposalStatus;
use crate::entity::proposal::{CreateProposal, Proposal, UpdateProposal};
use crate::entity::recruitment::CarPool;
use crate::entity::users::User;
use crate::error::Error;
use crate::repository::proposal::ProposalRepositoryTrait;
use crate::use_case::carpool_use_case::tests::{MockCarPoolRepo, MockCarPoolValue};
use crate::use_case::proposal_use_case::dto::AplProposal;
use crate::use_case::proposal_use_case::ProposalUseCase;
use axum::async_trait;
use chrono::{Duration, Utc};
use mockall::automock;
use std::sync::Arc;

#[automock]
pub trait ProposalValue {
    fn find(&self) -> Result<Proposal, Error>;
    fn find_by_carpool_id(&self) -> Result<Vec<Proposal>, Error>;
    fn create(&self) -> Result<Proposal, Error>;
    fn update(&self) -> Result<Proposal, Error>;
    fn update_proposal_status(&self) -> Result<Proposal, Error>;
    fn delete(&self) -> Result<(), Error>;
    fn find_by_user_and_carpool(&self) -> Result<Proposal, Error>;
}

pub struct MockProposalRepo {
    pub inner: MockProposalValue,
}

#[async_trait]
impl ProposalRepositoryTrait for MockProposalRepo {
    async fn find(&self, _id: &Id) -> Result<Proposal, Error> {
        self.inner.find()
    }
    async fn find_by_user_and_carpool(
        &self,
        _user_id: &User,
        _car_pool: &CarPool,
    ) -> Result<Proposal, Error> {
        self.inner.find_by_user_and_carpool()
    }
    async fn find_by_carpool_id(&self, _carpool_id: &Id) -> Result<Vec<Proposal>, Error> {
        self.inner.find_by_carpool_id()
    }
    async fn create(&self, _user: &User, _create: CreateProposal) -> Result<Proposal, Error> {
        self.inner.create()
    }
    async fn update(
        &self,
        _user: &User,
        _update_proposal: UpdateProposal,
    ) -> Result<Proposal, Error> {
        self.inner.update()
    }
    async fn update_proposal_status(&self, _status: ProposalStatus) -> Result<Proposal, Error> {
        self.inner.update_proposal_status()
    }
    async fn delete(&self, _id: &Id) -> Result<(), Error> {
        self.inner.delete()
    }
}

#[tokio::test]
async fn test_create() {
    let applicant = User {
        id: 100i64.try_into().unwrap(),
        ..User::default()
    };
    let input = AplProposal::default();
    // 正常系
    let mut pr = MockProposalValue::new();
    pr.expect_create().returning(|| Ok(Proposal::default()));
    pr.expect_find_by_user_and_carpool()
        .returning(|| Err(Error::NotFound("".to_string())));
    let mut cpr = MockCarPoolValue::new();
    cpr.expect_find_by().returning(|| Ok(valid_term_car_pool()));

    let uc = ProposalUseCase {
        pr: Arc::new(MockProposalRepo { inner: pr }),
        cpr: Arc::new(MockCarPoolRepo { inner: cpr }),
    };
    let res = uc.create(applicant.clone(), input.clone()).await;
    assert!(res.is_ok());

    // 募集が見つからない場合はエラー
    let mut pr = MockProposalValue::new();
    pr.expect_create().returning(|| Ok(Proposal::default()));
    pr.expect_find_by_user_and_carpool()
        .returning(|| Err(Error::NotFound("".to_string())));
    let mut cpr = MockCarPoolValue::new();
    cpr.expect_find_by()
        .returning(|| Err(Error::DbError("error".to_string())));
    let uc = ProposalUseCase {
        pr: Arc::new(MockProposalRepo { inner: pr }),
        cpr: Arc::new(MockCarPoolRepo { inner: cpr }),
    };
    let res = uc.create(applicant.clone(), input.clone()).await;
    assert!(res.is_err());

    // 申込者が主催者だった場合はエラー
    let mut pr = MockProposalValue::new();
    pr.expect_create().returning(|| Ok(Proposal::default()));
    pr.expect_find_by_user_and_carpool()
        .returning(|| Err(Error::NotFound("".to_string())));
    let mut cpr = MockCarPoolValue::new();
    cpr.expect_find_by().returning(|| {
        Ok(CarPool {
            organizer: User {
                id: 100i64.try_into().unwrap(),
                ..User::default()
            },
            ..valid_term_car_pool()
        })
    });
    let uc = ProposalUseCase {
        pr: Arc::new(MockProposalRepo { inner: pr }),
        cpr: Arc::new(MockCarPoolRepo { inner: cpr }),
    };
    let res = uc.create(applicant.clone(), input.clone()).await;
    assert!(res.is_err());

    // 申し込み期限日を過ぎてる場合はエラーを返す
    // テスト日の1日前
    let post_deadline: JaTimeStamp = (Utc::now() - Duration::days(1)).try_into().unwrap();
    let mut pr = MockProposalValue::new();
    pr.expect_create().returning(|| Ok(Proposal::default()));
    pr.expect_find_by_user_and_carpool()
        .returning(|| Err(Error::NotFound("".to_string())));
    let mut cpr = MockCarPoolValue::new();
    cpr.expect_find_by().returning(move || {
        Ok(CarPool {
            apl_deadline: post_deadline.clone(),
            ..CarPool::default()
        })
    });
    let uc = ProposalUseCase {
        pr: Arc::new(MockProposalRepo { inner: pr }),
        cpr: Arc::new(MockCarPoolRepo { inner: cpr }),
    };
    let res = uc.create(applicant.clone(), input.clone()).await;
    assert!(res.is_err());

    // すでに申し込みをしている場合はエラーを返す
    let mut pr = MockProposalValue::new();
    pr.expect_create().returning(|| Ok(Proposal::default()));
    pr.expect_find_by_user_and_carpool()
        .returning(|| Ok(Proposal::default()));
    let mut cpr = MockCarPoolValue::new();
    cpr.expect_find_by().returning(|| Ok(CarPool::default()));
    let uc = ProposalUseCase {
        pr: Arc::new(MockProposalRepo { inner: pr }),
        cpr: Arc::new(MockCarPoolRepo { inner: cpr }),
    };
    let res = uc.create(applicant.clone(), input.clone()).await;
    assert!(res.is_err())
}

fn valid_term_car_pool() -> CarPool {
    let now = (Utc::now() + Duration::days(1)).try_into().unwrap();
    CarPool {
        apl_deadline: now,
        ..CarPool::default()
    }
}
