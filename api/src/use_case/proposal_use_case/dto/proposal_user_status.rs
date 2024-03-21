use serde::{Serialize};
use std::fmt::{Display, Formatter};

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, Serialize, PartialOrd, PartialEq)]
pub enum ProposalUserStatus {
    Applying,
    CanApl,
    CannotApl,
    Owner,
}

impl Display for ProposalUserStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ProposalUserStatus::Applying => write!(f, "APPLYING"),
            ProposalUserStatus::CanApl => write!(f, "CAN_APL"),
            ProposalUserStatus::CannotApl => write!(f, "CANNOT_APL"),
            ProposalUserStatus::Owner => write!(f, "OWNER"),
        }
    }
}

#[cfg(test)]
impl Default for ProposalUserStatus {
    fn default() -> Self {
        Self::Applying
    }
}
