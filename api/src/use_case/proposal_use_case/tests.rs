use crate::domain::domain_object::carpool_status::CarPoolStatus;
use crate::domain::domain_object::id::Id;
use crate::domain::domain_object::ja_timestamp::JaTimeStamp;
use crate::domain::domain_object::proposal_status::ProposalStatus;
use crate::entity::car_pool::{CarPool, Point};
use crate::entity::proposal::{AcceptProposal, CreateProposal, Proposal, UpdateProposal};
use crate::entity::user::User;
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
    fn accept(&self) -> Result<Proposal, Error>;
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
    async fn update_proposal_status(
        &self,
        _id: Id,
        _status: ProposalStatus,
    ) -> Result<Proposal, Error> {
        self.inner.update_proposal_status()
    }
    async fn delete(&self, _id: &Id) -> Result<(), Error> {
        self.inner.delete()
    }
    async fn accept(&self, _accept_proposal: AcceptProposal) -> Result<Proposal, Error> {
        self.inner.accept()
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

#[tokio::test]
async fn test_cancel_by_applicant() {
    let applicant = User::default();
    let proposal_id: Id = 100i64.try_into().unwrap();

    // 正常系
    let mut pr = MockProposalValue::new();
    pr.expect_find().returning(|| {
        Ok(Proposal {
            carpool: CarPool {
                start_time: (Utc::now() + Duration::days(1)).try_into().unwrap(),
                ..CarPool::default()
            },
            ..applying_proposal()
        })
    });
    pr.expect_update_proposal_status()
        .returning(|| Ok(Proposal::default()));
    let cpr = MockCarPoolValue::new();

    let uc = ProposalUseCase {
        pr: Arc::new(MockProposalRepo { inner: pr }),
        cpr: Arc::new(MockCarPoolRepo { inner: cpr }),
    };
    let res = uc
        .cancel_by_applicant(applicant.clone(), proposal_id.clone())
        .await;
    assert!(res.is_ok());

    // 申し込みがない場合はエラー
    let mut pr = MockProposalValue::new();
    pr.expect_find()
        .returning(|| Err(Error::NotFound("".to_string())));
    pr.expect_update_proposal_status()
        .returning(|| Ok(Proposal::default()));
    let cpr = MockCarPoolValue::new();

    let uc = ProposalUseCase {
        pr: Arc::new(MockProposalRepo { inner: pr }),
        cpr: Arc::new(MockCarPoolRepo { inner: cpr }),
    };
    let res = uc
        .cancel_by_applicant(applicant.clone(), proposal_id.clone())
        .await;
    assert!(res.is_err());

    // 申込者とキャンセル依頼者が違う場合はエラー
    let mut pr = MockProposalValue::new();
    pr.expect_find().returning(|| {
        Ok(Proposal {
            carpool: CarPool {
                start_time: (Utc::now() + Duration::days(1)).try_into().unwrap(),
                ..CarPool::default()
            },
            user: User {
                id: 42i64.try_into().unwrap(),
                ..User::default()
            },
            ..applying_proposal()
        })
    });
    pr.expect_update_proposal_status()
        .returning(|| Ok(Proposal::default()));
    let cpr = MockCarPoolValue::new();

    let uc = ProposalUseCase {
        pr: Arc::new(MockProposalRepo { inner: pr }),
        cpr: Arc::new(MockCarPoolRepo { inner: cpr }),
    };
    let res = uc
        .cancel_by_applicant(applicant.clone(), proposal_id.clone())
        .await;
    assert!(res.is_err());

    // 出発日時を過ぎている場合はキャンセルできない
    let mut pr = MockProposalValue::new();
    pr.expect_find().returning(|| {
        Ok(Proposal {
            carpool: CarPool {
                start_time: (Utc::now() - Duration::days(1)).try_into().unwrap(),
                ..CarPool::default()
            },
            ..applying_proposal()
        })
    });
    pr.expect_update_proposal_status()
        .returning(|| Ok(Proposal::default()));
    let cpr = MockCarPoolValue::new();

    let uc = ProposalUseCase {
        pr: Arc::new(MockProposalRepo { inner: pr }),
        cpr: Arc::new(MockCarPoolRepo { inner: cpr }),
    };
    let res = uc
        .cancel_by_applicant(applicant.clone(), proposal_id.clone())
        .await;
    assert!(res.is_err());

    // 申し込みの更新に失敗した場合はエラーを返す
    let mut pr = MockProposalValue::new();
    pr.expect_find().returning(|| {
        Ok(Proposal {
            carpool: CarPool {
                start_time: (Utc::now() + Duration::days(1)).try_into().unwrap(),
                ..CarPool::default()
            },
            ..applying_proposal()
        })
    });
    pr.expect_update_proposal_status()
        .returning(|| Err(Error::DbError("".to_string())));
    let cpr = MockCarPoolValue::new();

    let uc = ProposalUseCase {
        pr: Arc::new(MockProposalRepo { inner: pr }),
        cpr: Arc::new(MockCarPoolRepo { inner: cpr }),
    };
    let res = uc
        .cancel_by_applicant(applicant.clone(), proposal_id.clone())
        .await;
    assert!(res.is_err());

    // すでにキャンセルされている場合はエラー
    let mut pr = MockProposalValue::new();
    pr.expect_find().returning(|| {
        Ok(Proposal {
            carpool: CarPool {
                start_time: (Utc::now() + Duration::days(1)).try_into().unwrap(),
                ..CarPool::default()
            },
            status: ProposalStatus::UserCancel,
            ..applying_proposal()
        })
    });
    pr.expect_update_proposal_status()
        .returning(|| Ok(Proposal::default()));
    let cpr = MockCarPoolValue::new();

    let uc = ProposalUseCase {
        pr: Arc::new(MockProposalRepo { inner: pr }),
        cpr: Arc::new(MockCarPoolRepo { inner: cpr }),
    };
    let res = uc
        .cancel_by_applicant(applicant.clone(), proposal_id.clone())
        .await;
    assert!(res.is_err());
}

fn applying_proposal() -> Proposal {
    Proposal {
        status: ProposalStatus::Applying,
        ..Proposal::default()
    }
}

#[tokio::test]
async fn test_accept_proposal() {
    let organizer = User::default();
    let accept_proposal = AcceptProposal::default();

    // 正常系
    let mut pr = MockProposalValue::new();
    pr.expect_find().returning(|| {
        Ok(Proposal {
            carpool: CarPool {
                start_time: (Utc::now() + Duration::days(2)).try_into().unwrap(),
                ..CarPool::default()
            },
            ..applying_proposal()
        })
    });
    pr.expect_accept().returning(|| Ok(Proposal::default()));
    let mut cpr = MockCarPoolValue::new();
    cpr.expect_update().returning(|| Ok(CarPool::default()));

    let uc = ProposalUseCase {
        pr: Arc::new(MockProposalRepo { inner: pr }),
        cpr: Arc::new(MockCarPoolRepo { inner: cpr }),
    };
    let res = uc
        .accept_proposal(organizer.clone(), accept_proposal.clone())
        .await;
    println!("{res:?}");
    assert!(res.is_ok());

    // Proposalが存在しない場合はエラー
    let mut pr = MockProposalValue::new();
    pr.expect_find()
        .returning(|| Err(Error::DbError("".to_string())));
    pr.expect_accept().returning(|| Ok(Proposal::default()));
    let mut cpr = MockCarPoolValue::new();
    cpr.expect_update().returning(|| Ok(CarPool::default()));

    let uc = ProposalUseCase {
        pr: Arc::new(MockProposalRepo { inner: pr }),
        cpr: Arc::new(MockCarPoolRepo { inner: cpr }),
    };
    let res = uc
        .accept_proposal(organizer.clone(), accept_proposal.clone())
        .await;
    println!("{res:?}");
    assert!(res.is_err());

    // Proposalの更新に失敗した場合はエラー
    let mut pr = MockProposalValue::new();
    pr.expect_find().returning(|| {
        Ok(Proposal {
            carpool: CarPool {
                start_time: (Utc::now() + Duration::days(2)).try_into().unwrap(),
                ..CarPool::default()
            },
            ..applying_proposal()
        })
    });
    pr.expect_accept()
        .returning(|| Err(Error::DbError("".to_string())));
    let mut cpr = MockCarPoolValue::new();
    cpr.expect_update().returning(|| Ok(CarPool::default()));

    let uc = ProposalUseCase {
        pr: Arc::new(MockProposalRepo { inner: pr }),
        cpr: Arc::new(MockCarPoolRepo { inner: cpr }),
    };
    let res = uc
        .accept_proposal(organizer.clone(), accept_proposal.clone())
        .await;
    println!("{res:?}");
    assert!(res.is_err());

    // CarPoolの更新に失敗した場合はエラー
    let mut pr = MockProposalValue::new();
    pr.expect_find().returning(|| {
        Ok(Proposal {
            carpool: CarPool {
                start_time: (Utc::now() + Duration::days(2)).try_into().unwrap(),
                ..CarPool::default()
            },
            ..applying_proposal()
        })
    });
    pr.expect_accept().returning(|| Ok(Proposal::default()));
    let mut cpr = MockCarPoolValue::new();
    cpr.expect_update()
        .returning(|| Err(Error::DbError("".to_string())));

    let uc = ProposalUseCase {
        pr: Arc::new(MockProposalRepo { inner: pr }),
        cpr: Arc::new(MockCarPoolRepo { inner: cpr }),
    };
    let res = uc
        .accept_proposal(organizer.clone(), accept_proposal.clone())
        .await;
    println!("{res:?}");
    assert!(res.is_err());

    // 開始日の1日前より後に承認しようとする場合はエラー
    let mut pr = MockProposalValue::new();
    pr.expect_find().returning(|| {
        Ok(Proposal {
            carpool: CarPool {
                start_time: (Utc::now() + Duration::hours(12)).try_into().unwrap(),
                ..CarPool::default()
            },
            ..applying_proposal()
        })
    });
    pr.expect_accept().returning(|| Ok(Proposal::default()));
    let mut cpr = MockCarPoolValue::new();
    cpr.expect_update().returning(|| Ok(CarPool::default()));

    let uc = ProposalUseCase {
        pr: Arc::new(MockProposalRepo { inner: pr }),
        cpr: Arc::new(MockCarPoolRepo { inner: cpr }),
    };
    let res = uc
        .accept_proposal(organizer.clone(), accept_proposal.clone())
        .await;
    println!("{res:?}");
    assert!(res.is_err());

    // 主催者以外が承認しようとした場合はエラー
    let another_organizer = User {
        id: 42i64.try_into().unwrap(),
        ..User::default()
    };
    let mut pr = MockProposalValue::new();
    pr.expect_find().returning(|| {
        Ok(Proposal {
            carpool: CarPool {
                start_time: (Utc::now() + Duration::days(2)).try_into().unwrap(),
                ..CarPool::default()
            },
            ..applying_proposal()
        })
    });
    pr.expect_accept().returning(|| Ok(Proposal::default()));
    let mut cpr = MockCarPoolValue::new();
    cpr.expect_update().returning(|| Ok(CarPool::default()));

    let uc = ProposalUseCase {
        pr: Arc::new(MockProposalRepo { inner: pr }),
        cpr: Arc::new(MockCarPoolRepo { inner: cpr }),
    };
    let res = uc
        .accept_proposal(another_organizer.clone(), accept_proposal.clone())
        .await;
    println!("{res:?}");
    assert!(res.is_err());

    // ピックアップポイントが不正な場合はエラー
    let invalid_accept_proposal = AcceptProposal {
        pick_up_point: Point {
            municipality: "invalid municipality".to_string().try_into().unwrap(),
            ..Point::default()
        },
        ..AcceptProposal::default()
    };
    let res = uc
        .accept_proposal(organizer.clone(), invalid_accept_proposal.clone())
        .await;
    println!("{res:?}");
    assert!(res.is_err());

    // すでに参加人数が集まっている場合はエラー
    let mut pr = MockProposalValue::new();
    pr.expect_find().returning(|| {
        Ok(Proposal {
            carpool: CarPool {
                start_time: (Utc::now() + Duration::days(2)).try_into().unwrap(),
                status: CarPoolStatus::AplComplete,
                ..CarPool::default()
            },
            ..applying_proposal()
        })
    });
    pr.expect_accept().returning(|| Ok(Proposal::default()));
    let mut cpr = MockCarPoolValue::new();
    cpr.expect_update().returning(|| Ok(CarPool::default()));

    let uc = ProposalUseCase {
        pr: Arc::new(MockProposalRepo { inner: pr }),
        cpr: Arc::new(MockCarPoolRepo { inner: cpr }),
    };
    let res = uc
        .accept_proposal(organizer.clone(), accept_proposal.clone())
        .await;
    println!("{res:?}");
    assert!(res.is_err());
}

#[tokio::test]
async fn test_edit_proposal() {
    let user = User::default();
    let another_user = User {
        id: 42i64.try_into().unwrap(),
        ..User::default()
    };
    let input = UpdateProposal::default();

    // 正常系
    let mut pr = MockProposalValue::new();
    pr.expect_find().returning(|| Ok(applying_proposal()));
    pr.expect_update().returning(|| Ok(Proposal::default()));
    let cpr = MockCarPoolValue::new();

    let uc = ProposalUseCase {
        pr: Arc::new(MockProposalRepo { inner: pr }),
        cpr: Arc::new(MockCarPoolRepo { inner: cpr }),
    };
    let res = uc.edit_proposal(user.clone(), input.clone()).await;
    assert!(res.is_ok());

    // 申込者以外のユーザーが更新しようとした場合
    let res = uc.edit_proposal(another_user.clone(), input.clone()).await;
    assert!(res.is_err());

    // 承認前でない場合
    let mut pr = MockProposalValue::new();
    pr.expect_find().returning(|| {
        Ok(Proposal {
            status: ProposalStatus::Acceptance,
            ..Proposal::default()
        })
    });
    pr.expect_update().returning(|| Ok(Proposal::default()));
    let cpr = MockCarPoolValue::new();
    let uc = ProposalUseCase {
        pr: Arc::new(MockProposalRepo { inner: pr }),
        cpr: Arc::new(MockCarPoolRepo { inner: cpr }),
    };
    let res = uc.edit_proposal(user.clone(), input.clone()).await;
    assert!(res.is_err());

    // 申し込みが存在しない場合
    let mut pr = MockProposalValue::new();
    pr.expect_find()
        .returning(|| Err(Error::NotFound("".to_string())));
    pr.expect_update().returning(|| Ok(Proposal::default()));
    let cpr = MockCarPoolValue::new();
    let uc = ProposalUseCase {
        pr: Arc::new(MockProposalRepo { inner: pr }),
        cpr: Arc::new(MockCarPoolRepo { inner: cpr }),
    };
    let res = uc.edit_proposal(user.clone(), input.clone()).await;
    assert!(res.is_err());

    // 更新に失敗した場合
    let mut pr = MockProposalValue::new();
    pr.expect_find().returning(|| Ok(applying_proposal()));
    pr.expect_update()
        .returning(|| Err(Error::DbError("".to_string())));
    let cpr = MockCarPoolValue::new();
    let uc = ProposalUseCase {
        pr: Arc::new(MockProposalRepo { inner: pr }),
        cpr: Arc::new(MockCarPoolRepo { inner: cpr }),
    };
    let res = uc.edit_proposal(user.clone(), input.clone()).await;
    assert!(res.is_err());
}
