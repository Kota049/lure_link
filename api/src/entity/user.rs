use crate::domain::domain_object::application_token::ApplicationToken;
use crate::domain::domain_object::first_name::FirstName;
use crate::domain::domain_object::id::Id;
use crate::domain::domain_object::last_name::LastName;
use crate::domain::domain_object::name::Name;
use crate::domain::domain_object::name_jp::NameJp;
use crate::domain::domain_object::user_status::UserStatus;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialOrd, PartialEq)]
pub struct User {
    pub id: Id,
    pub application_token: ApplicationToken,
    pub application_refresh_token: ApplicationToken,
    pub line_access_token: String,
    pub line_refresh_token: String,
    pub line_id: String,
    pub nick_name: Option<Name>,
    pub first_name: Option<FirstName>,
    pub last_name: Option<LastName>,
    pub first_name_jp: Option<NameJp>,
    pub last_name_jp: Option<NameJp>,
    pub user_status: UserStatus,
}

impl Default for User {
    fn default() -> Self {
        User {
            id: 1.try_into().unwrap(),
            application_token: "a".to_string().try_into().unwrap(),
            application_refresh_token: "a".to_string().try_into().unwrap(),
            line_access_token: "".to_string(),
            line_refresh_token: "".to_string(),
            line_id: "".to_string(),
            nick_name: None,
            first_name: None,
            last_name: None,
            first_name_jp: None,
            last_name_jp: None,
            user_status: UserStatus::Trial,
        }
    }
}
