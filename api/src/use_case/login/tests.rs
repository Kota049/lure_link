use std::sync::Arc;
use axum::async_trait;
use mockall::automock;
use crate::entity::line::{LineProfile, LineToken};
use crate::entity::users::User;
use crate::error::Error;
use crate::repository::line::LineClientTrait;
use crate::repository::user::UserRepositoryTrait;
use crate::use_case::login::LoginUseCase;

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
    async fn get_token(&self, _code: &str) -> Result<LineToken, Error> { self.inner.line_token() }
    async fn get_profile(&self, _line_token: LineToken) -> Result<LineProfile, Error> { self.inner.line_profile() }
}

#[automock]
pub trait UserValue {
    fn create_res(&self) -> Result<User, Error>;
    fn user_by_application_token(&self) -> Result<User, Error>;
}

struct MockUserRepo {
    inner: MockUserValue,
}

#[async_trait]
impl UserRepositoryTrait for MockUserRepo {
    async fn find_by_application_token(&self, _token: &String) -> Result<User, Error> { self.inner.user_by_application_token() }
    async fn create(&self, _line_token: &LineToken, _line_profile: &LineProfile) -> Result<User, Error> { self.inner.create_res() }
}

#[tokio::test]
async fn test_verify_user() {
    let valid_code = "valid code".to_string();
    // 正常な場合はUser情報を返す
    let mut lc = MockLineValue::new();
    lc.expect_line_token().returning(|| Ok(LineToken {
        access_token: "valid".to_string(),
        expires_in: 0,
        refresh_token: "valid".to_string(),
        id_token: "valid".to_string(),
    }));
    lc.expect_line_profile().returning(|| Ok(LineProfile { line_user_id: "valid".to_string() }));

    let mut ur = MockUserValue::new();
    ur.expect_create_res().returning(|| Ok(User {
        id: 1i64.try_into().unwrap(),
        application_token: "".to_string(),
        application_refresh_token: "".to_string(),
        line_access_token: "".to_string(),
        line_refresh_token: "".to_string(),
        line_id: "".to_string(),
    }));

    let uc = LoginUseCase { u_repo: Arc::new(MockUserRepo { inner: ur }), line_client: Arc::new(MockLineRepo { inner: lc }) };
    let res = uc.verify_user(valid_code.clone()).await;
    assert!(res.is_ok());

    // 正常なprofileが取得できなかった場合にはエラーを返す
    let mut lc = MockLineValue::new();
    lc.expect_line_token().returning(|| Err(Error::LineError("test line error".to_string())));
    lc.expect_line_profile().returning(|| Ok(LineProfile { line_user_id: "valid".to_string() }));

    let mut ur = MockUserValue::new();
    ur.expect_create_res().returning(|| Ok(User {
        id: 1i64.try_into().unwrap(),
        application_token: "".to_string(),
        application_refresh_token: "".to_string(),
        line_access_token: "".to_string(),
        line_refresh_token: "".to_string(),
        line_id: "".to_string(),
    }));

    let uc = LoginUseCase { u_repo: Arc::new(MockUserRepo { inner: ur }), line_client: Arc::new(MockLineRepo { inner: lc }) };
    let res = uc.verify_user(valid_code).await;
    assert!(res.is_err());
}