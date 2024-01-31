use crate::error::Error;
use serde::de::Error as DeError;
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt::{Display, Formatter};
use tokio_postgres::types::private::BytesMut;
use tokio_postgres::types::{FromSql, IsNull, ToSql, Type};

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, Serialize, PartialOrd, PartialEq)]
pub enum ProposalStatus {
    Applying,
    Acceptance,
    Deny,
    UserCancel,
    OrganizerCancel,
}

impl TryFrom<String> for ProposalStatus {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "APPLYING" => Ok(Self::Applying),
            "ACCEPTANCE" => Ok(Self::Acceptance),
            "DENY" => Ok(Self::Deny),
            "USER_CANCEL" => Ok(Self::UserCancel),
            "ORGANIZER_CANCEL" => Ok(Self::OrganizerCancel),
            _ => Err(Error::ValidateError("invalid proposal status".to_string())),
        }
    }
}

impl<'a> FromSql<'a> for ProposalStatus {
    fn from_sql(
        type_: &Type,
        raw: &'a [u8],
    ) -> Result<Self, Box<dyn std::error::Error + 'static + Sync + Send>> {
        let s: String = FromSql::from_sql(type_, raw)?;
        Ok(s.try_into()
            .map_err(|_| Box::new(Error::ValidateError("invalid proposal status".to_string())))?)
    }
    fn accepts(_type_: &Type) -> bool {
        true
    }
}

impl Display for ProposalStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ProposalStatus::Applying => write!(f, "APPLYING"),
            ProposalStatus::Acceptance => write!(f, "ACCEPTANCE"),
            ProposalStatus::Deny => write!(f, "DENY"),
            ProposalStatus::UserCancel => write!(f, "USER_CANCEL"),
            ProposalStatus::OrganizerCancel => write!(f, "ORGANIZER_CANCEL"),
        }
    }
}

impl ToSql for ProposalStatus {
    fn to_sql(
        &self,
        _ty: &Type,
        out: &mut BytesMut,
    ) -> Result<IsNull, Box<dyn std::error::Error + 'static + Sync + Send>> {
        let prefecture = self.to_string();
        out.extend_from_slice(prefecture.as_bytes());
        Ok(IsNull::No)
    }
    fn accepts(_type_: &Type) -> bool {
        true
    }
    tokio_postgres::types::to_sql_checked!();
}

impl<'de> Deserialize<'de> for ProposalStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        value
            .try_into()
            .map_err(|_| D::Error::custom("validate proposal status error"))
    }
}

#[cfg(test)]
impl Default for ProposalStatus {
    fn default() -> Self {
        Self::Applying
    }
}
