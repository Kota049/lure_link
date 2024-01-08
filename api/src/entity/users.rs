use serde::{Deserialize, Serialize};
use crate::domain::domain_object::id::Id;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Id,
    pub application_token: String,
    pub application_refresh_token: String,
    pub line_access_token: String,
    pub line_refresh_token: String,
    pub line_id: String,
}