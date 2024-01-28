use crate::entity::line::{LineProfile, LineToken};
use crate::entity::users::User;
use crate::error::Error;
use crate::error::Error::{DbError, NotFound};
use crate::repository::line::LineClientTrait;
use crate::repository::user::UserRepositoryTrait;
use crate::use_case::login::LoginUseCase;
use axum::async_trait;
use mockall::automock;
use std::sync::Arc;

#[automock]
pub trait LineValue {
    fn line_token(&self) -> Result<LineToken, Error>;
    fn line_profile(&self) -> Result<LineProfile, Error>;
}

struct MockLineRepo {
    inner: MockLineValue,
}

#[async_trait]
impl LineClientTrait for MockLineRepo {
    async fn get_token(&self, _code: &str) -> Result<LineToken, Error> {
        self.inner.line_token()
    }
    async fn get_profile(&self, _line_token: LineToken) -> Result<LineProfile, Error> {
        self.inner.line_profile()
    }
}

#[automock]
pub trait UserValue {
    fn create_res(&self) -> Result<User, Error>;
    fn user_by_application_token(&self) -> Result<User, Error>;
    fn user_by_line_token(&self) -> Result<User, Error>;
}

struct MockUserRepo {
    inner: MockUserValue,
}

#[async_trait]
impl UserRepositoryTrait for MockUserRepo {
    async fn find_by_application_token(&self, _token: &String) -> Result<User, Error> {
        self.inner.user_by_application_token()
    }
    async fn create(
        &self,
        _line_token: &LineToken,
        _line_profile: &LineProfile,
    ) -> Result<User, Error> {
        self.inner.create_res()
    }

    async fn find_by_line_token(&self, line_id: &String) -> Result<User, Error> {
        self.inner.user_by_line_token()
    }
}

#[tokio::test]
async fn test_verify_user() {
    let valid_code = "valid code".to_string();
    // 正常な場合はUser情報を返す
    let lc = create_lc();
    let ur = create_ur();
    let uc = LoginUseCase {
        u_repo: Arc::new(MockUserRepo { inner: ur }),
        line_client: Arc::new(MockLineRepo { inner: lc }),
    };
    let res = uc.verify_user(valid_code.clone()).await;
    assert!(res.is_ok());

    // lineのtokenが取得できなかった場合にはエラーを返す
    let mut lc = MockLineValue::new();
    lc.expect_line_token()
        .returning(|| Err(Error::LineError("test line error".to_string())));
    lc.expect_line_profile().returning(|| {
        Ok(LineProfile {
            line_user_id: "valid".to_string(),
        })
    });
    let ur = create_ur();
    let uc = LoginUseCase {
        u_repo: Arc::new(MockUserRepo { inner: ur }),
        line_client: Arc::new(MockLineRepo { inner: lc }),
    };
    let res = uc.verify_user(valid_code.clone()).await;
    assert!(res.is_err());

    // profileが取得できなかった場合にはエラーを返す
    let mut lc = MockLineValue::new();
    lc.expect_line_token().returning(|| {
        Ok(LineToken {
            access_token: "valid".to_string(),
            expires_in: 0,
            refresh_token: "valid".to_string(),
            id_token: "valid".to_string(),
        })
    });
    lc.expect_line_profile()
        .returning(|| Err(Error::LineError("test line error".to_string())));
    let ur = create_ur();
    let uc = LoginUseCase {
        u_repo: Arc::new(MockUserRepo { inner: ur }),
        line_client: Arc::new(MockLineRepo { inner: lc }),
    };
    let res = uc.verify_user(valid_code).await;
    assert!(res.is_err());
}

fn create_lc() -> MockLineValue {
    let mut lc = MockLineValue::new();
    lc.expect_line_token().returning(|| {
        Ok(LineToken {
            access_token: "valid".to_string(),
            expires_in: 0,
            refresh_token: "valid".to_string(),
            id_token: "valid".to_string(),
        })
    });
    lc.expect_line_profile().returning(|| {
        Ok(LineProfile {
            line_user_id: "valid".to_string(),
        })
    });
    lc
}

#[tokio::test]
async fn test_upsert_token() {
    // line_idからデータが取得できた場合にはそのUserを返す
    let lc = create_lc();
    let ur = create_ur();
    let uc = LoginUseCase {
        u_repo: Arc::new(MockUserRepo { inner: ur }),
        line_client: Arc::new(MockLineRepo { inner: lc }),
    };
    let line_token = LineToken {
        access_token: "".to_string(),
        expires_in: 0,
        refresh_token: "".to_string(),
        id_token: "".to_string(),
    };
    let line_profile = LineProfile {
        line_user_id: "".to_string(),
    };
    let res = uc
        .find_or_create_user(line_token, line_profile)
        .await
        .unwrap();
    assert_eq!(res, create_ur().user_by_line_token().unwrap());

    // line_idからデータが見つからない場合はデータを作成する
    let lc = create_lc();
    let mut ur = MockUserValue::new();
    ur.expect_user_by_line_token()
        .returning(|| Err(NotFound("not found".to_string())));
    ur.expect_create_res().returning(|| Ok(create_some_user()));
    let uc = LoginUseCase {
        u_repo: Arc::new(MockUserRepo { inner: ur }),
        line_client: Arc::new(MockLineRepo { inner: lc }),
    };
    let line_token = LineToken {
        access_token: "".to_string(),
        expires_in: 0,
        refresh_token: "".to_string(),
        id_token: "".to_string(),
    };
    let line_profile = LineProfile {
        line_user_id: "".to_string(),
    };
    let res = uc
        .find_or_create_user(line_token, line_profile)
        .await
        .unwrap();
    assert_eq!(res, create_some_user());

    // ユーザー検索の際に予期せぬエラーが発生した場合はエラーを返す
    let lc = create_lc();
    let mut ur = MockUserValue::new();
    ur.expect_user_by_line_token()
        .returning(|| Err(DbError("some problem occurs".to_string())));
    ur.expect_create_res().returning(|| Ok(create_some_user()));
    let uc = LoginUseCase {
        u_repo: Arc::new(MockUserRepo { inner: ur }),
        line_client: Arc::new(MockLineRepo { inner: lc }),
    };
    let line_token = LineToken {
        access_token: "".to_string(),
        expires_in: 0,
        refresh_token: "".to_string(),
        id_token: "".to_string(),
    };
    let line_profile = LineProfile {
        line_user_id: "".to_string(),
    };
    let res = uc.find_or_create_user(line_token, line_profile).await;
    assert!(res.is_err());

    // ユーザーの作成に失敗した場合にはエラーを返す
    let lc = create_lc();
    let mut ur = MockUserValue::new();
    ur.expect_user_by_line_token()
        .returning(|| Err(DbError("error".to_string())));
    ur.expect_create_res()
        .returning(|| Err(DbError("error".to_string())));
    let uc = LoginUseCase {
        u_repo: Arc::new(MockUserRepo { inner: ur }),
        line_client: Arc::new(MockLineRepo { inner: lc }),
    };
    let line_token = LineToken {
        access_token: "".to_string(),
        expires_in: 0,
        refresh_token: "".to_string(),
        id_token: "".to_string(),
    };
    let line_profile = LineProfile {
        line_user_id: "".to_string(),
    };
    let res = uc.find_or_create_user(line_token, line_profile).await;
    assert!(res.is_err());
}

#[tokio::test]
async fn test_verify_token() {
    // application_tokenからデータが取得できた場合にはそのUserを返す
    let lc = create_lc();
    let ur = create_ur();
    let uc = LoginUseCase {
        u_repo: Arc::new(MockUserRepo { inner: ur }),
        line_client: Arc::new(MockLineRepo { inner: lc }),
    };
    let application_token = "valid_token".to_string().try_into().unwrap();
    let res = uc.verify_token(&application_token).await;
    assert!(res.is_ok());

    // ユーザーが存在しない場合エラーを返す
    let lc = create_lc();
    let mut ur = MockUserValue::new();
    ur.expect_user_by_application_token()
        .returning(|| Err(NotFound("not found".to_string())));
    let uc = LoginUseCase {
        u_repo: Arc::new(MockUserRepo { inner: ur }),
        line_client: Arc::new(MockLineRepo { inner: lc }),
    };
    let application_token = "valid_token".to_string().try_into().unwrap();
    let res = uc.verify_token(&application_token).await;
    assert!(res.is_err());

    // DB接続等でエラーが発生した場合はエラーを返す
    let lc = create_lc();
    let mut ur = MockUserValue::new();
    ur.expect_user_by_application_token()
        .returning(|| Err(DbError("some db error occurs".to_string())));
    let uc = LoginUseCase {
        u_repo: Arc::new(MockUserRepo { inner: ur }),
        line_client: Arc::new(MockLineRepo { inner: lc }),
    };
    let application_token = "valid_token".to_string().try_into().unwrap();
    let res = uc.verify_token(&application_token).await;
    assert!(res.is_err());
}

fn create_some_user() -> User {
    User {
        id: 100.try_into().unwrap(),
        application_token: "test not found".to_string().try_into().unwrap(),
        application_refresh_token: "test not found".to_string().try_into().unwrap(),
        line_access_token: "".to_string(),
        line_refresh_token: "".to_string(),
        line_id: "".to_string(),
    }
}

fn create_ur() -> MockUserValue {
    let mut ur = MockUserValue::new();
    ur.expect_create_res().returning(|| {
        Ok(User {
            id: 1i64.try_into().unwrap(),
            application_token: "a".to_string().try_into().unwrap(),
            application_refresh_token: "a".to_string().try_into().unwrap(),
            line_access_token: "".to_string(),
            line_refresh_token: "".to_string(),
            line_id: "".to_string(),
        })
    });
    ur.expect_user_by_line_token().returning(|| {
        Ok(User {
            id: 2i64.try_into().unwrap(),
            application_token: "a".to_string().try_into().unwrap(),
            application_refresh_token: "a".to_string().try_into().unwrap(),
            line_access_token: "".to_string(),
            line_refresh_token: "".to_string(),
            line_id: "".to_string(),
        })
    });
    ur
}
