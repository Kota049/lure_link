use crate::domain::domain_object::application_token::ApplicationToken;
use crate::domain::domain_object::id::Id;
// use crate::domain::domain_object::name::Name;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialOrd, PartialEq)]
pub struct User {
    pub id: Id,
    pub application_token: ApplicationToken,
    pub application_refresh_token: ApplicationToken,
    pub line_access_token: String,
    pub line_refresh_token: String,
    pub line_id: String,
    // pub nick_name: Name,
    // pub first_name: Name,
}
