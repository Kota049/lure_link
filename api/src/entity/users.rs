use crate::domain::domain_object::application_token::ApplicationToken;
use crate::domain::domain_object::id::Id;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Id,
    pub application_token: ApplicationToken,
    pub application_refresh_token: ApplicationToken,
    pub line_access_token: String,
    pub line_refresh_token: String,
    pub line_id: String,
}
