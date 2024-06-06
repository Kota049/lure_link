use crate::domain::domain_object::application_token::ApplicationToken;
use crate::entity::line::{LineProfile, LineToken};
use crate::entity::user::User;
use crate::error::Error;
use crate::error::Error::{DbError, NotFound};
use crate::repository::line::LineClientTrait;
use crate::repository::user::UserRepositoryTrait;
use crate::use_case::user_use_case::dto::UpdateUser;
use crate::use_case::user_use_case::UserUseCase;
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
    fn update_token(&self) -> Result<User, Error>;
    fn register_user(&self) -> Result<User, Error>;
    fn save(&self) -> Result<User, Error>;
}

pub struct MockUserRepo {
    pub(crate) inner: MockUserValue,
}

#[async_trait]
impl UserRepositoryTrait for MockUserRepo {
    async fn find_by_application_token(&self, _token: &ApplicationToken) -> Result<User, Error> {
        self.inner.user_by_application_token()
    }
    async fn create(
        &self,
        _line_token: &LineToken,
        _line_profile: &LineProfile,
    ) -> Result<User, Error> {
        self.inner.create_res()
    }

    async fn find_by_line_id(&self, _line_id: &String) -> Result<User, Error> {
        self.inner.user_by_line_token()
    }

    async fn update_token(&self, _refresh_token: &ApplicationToken) -> Result<User, Error> {
        self.inner.update_token()
    }
    async fn register_user(&self, _up: UpdateUser) -> Result<User, Error> {
        self.inner.register_user()
    }

    async fn save(&self, user: User) -> Result<User, Error> {
        self.inner.save()
    }
}

#[tokio::test]
async fn test_verify_user() {
    let valid_code = "valid code".to_string();
    // 正常な場合はUser情報を返す
    let lc = create_lc();
    let ur = create_ur();
    let uc = UserUseCase {
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
    let uc = UserUseCase {
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
    let uc = UserUseCase {
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
    let uc = UserUseCase {
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
    let uc = UserUseCase {
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
    let uc = UserUseCase {
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
    let uc = UserUseCase {
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
    let uc = UserUseCase {
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
    let uc = UserUseCase {
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
    let uc = UserUseCase {
        u_repo: Arc::new(MockUserRepo { inner: ur }),
        line_client: Arc::new(MockLineRepo { inner: lc }),
    };
    let application_token = "valid_token".to_string().try_into().unwrap();
    let res = uc.verify_token(&application_token).await;
    assert!(res.is_err());
}

#[tokio::test]
async fn test_refresh_token() {
    // refresh_tokenが存在する場合はトークンを更新する
    let lc = create_lc();
    let ur = create_ur();
    let uc = UserUseCase {
        u_repo: Arc::new(MockUserRepo { inner: ur }),
        line_client: Arc::new(MockLineRepo { inner: lc }),
    };
    let application_token = "valid_token".to_string().try_into().unwrap();
    let res = uc.refresh_token(&application_token).await;
    assert!(res.is_ok());

    // tokenが存在しない場合はエラーを返す
    let lc = create_lc();
    let mut ur = MockUserValue::new();
    ur.expect_update_token()
        .returning(|| Err(NotFound("not found".to_string())));
    let uc = UserUseCase {
        u_repo: Arc::new(MockUserRepo { inner: ur }),
        line_client: Arc::new(MockLineRepo { inner: lc }),
    };
    let application_token = "valid_token".to_string().try_into().unwrap();
    let res = uc.refresh_token(&application_token).await;
    assert!(res.is_err());

    // DB接続等でエラーが発生した場合はエラーを返す
    let lc = create_lc();
    let mut ur = MockUserValue::new();
    ur.expect_update_token()
        .returning(|| Err(DbError("some db error occurs".to_string())));
    let uc = UserUseCase {
        u_repo: Arc::new(MockUserRepo { inner: ur }),
        line_client: Arc::new(MockLineRepo { inner: lc }),
    };
    let application_token = "valid_token".to_string().try_into().unwrap();
    let res = uc.refresh_token(&application_token).await;
    assert!(res.is_err());
}

#[tokio::test]
async fn test_sign_up() {
    // 正常な場合はUserを作成して返す
    let lc = create_lc();
    let ur = create_ur();
    let uc = UserUseCase {
        u_repo: Arc::new(MockUserRepo { inner: ur }),
        line_client: Arc::new(MockLineRepo { inner: lc }),
    };
    let up = UpdateUser::default();
    let res = uc.first_register_user(up).await;
    assert!(res.is_ok());

    // バリデーションに引っかかる場合
    let up = UpdateUser {
        last_name: None,
        ..UpdateUser::default()
    };
    let res = uc.first_register_user(up).await;
    assert!(res.is_err());

    // トークンが無効な場合はエラーを返す
    let lc = create_lc();
    let mut ur = MockUserValue::new();
    ur.expect_user_by_application_token()
        .returning(|| Err(DbError("error".to_string())));
    let uc = UserUseCase {
        u_repo: Arc::new(MockUserRepo { inner: ur }),
        line_client: Arc::new(MockLineRepo { inner: lc }),
    };
    let up = UpdateUser::default();
    let res = uc.first_register_user(up).await;
    assert!(res.is_err());

    // 登録に失敗した場合
    let lc = create_lc();
    let mut ur = MockUserValue::new();
    ur.expect_user_by_application_token()
        .returning(|| Ok(User::default()));
    ur.expect_register_user()
        .returning(|| Err(DbError("error".to_string())));
    let uc = UserUseCase {
        u_repo: Arc::new(MockUserRepo { inner: ur }),
        line_client: Arc::new(MockLineRepo { inner: lc }),
    };
    let up = UpdateUser::default();
    let res = uc.first_register_user(up).await;
    assert!(res.is_err())
}

fn create_some_user() -> User {
    User {
        id: 100.try_into().unwrap(),
        ..User::default()
    }
}

fn create_ur() -> MockUserValue {
    let mut ur = MockUserValue::new();
    ur.expect_create_res().returning(|| {
        Ok(User {
            id: 1i64.try_into().unwrap(),
            ..User::default()
        })
    });
    ur.expect_user_by_line_token().returning(|| {
        Ok(User {
            id: 2i64.try_into().unwrap(),
            ..User::default()
        })
    });
    ur.expect_user_by_application_token().returning(|| {
        Ok(User {
            id: 3i64.try_into().unwrap(),
            ..User::default()
        })
    });
    ur.expect_update_token().returning(|| {
        Ok(User {
            id: 4i64.try_into().unwrap(),
            ..User::default()
        })
    });
    ur.expect_register_user().returning(|| {
        Ok(User {
            id: 5i64.try_into().unwrap(),
            ..User::default()
        })
    });
    ur
}
