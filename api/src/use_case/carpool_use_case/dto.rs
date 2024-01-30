use crate::domain::domain_object::id::Id;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CancelCarPool {
    pub id: Id,
}
