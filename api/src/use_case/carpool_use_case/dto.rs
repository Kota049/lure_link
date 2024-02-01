use crate::domain::domain_object::id::Id;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CancelCarPool {
    pub id: Id,
}

#[cfg(test)]
impl Default for CancelCarPool {
    fn default() -> Self {
        Self {
            id: 1i64.try_into().unwrap(),
        }
    }
}
