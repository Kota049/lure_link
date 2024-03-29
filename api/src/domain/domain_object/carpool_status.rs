use crate::error::Error;
use serde::de::Error as DeError;
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt::{Display, Formatter};
use tokio_postgres::types::private::BytesMut;
use tokio_postgres::types::{FromSql, IsNull, ToSql, Type};

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, Serialize, PartialOrd, PartialEq)]
pub enum CarPoolStatus {
    Applying,
    AplComplete,
    Cancel,
    Finished,
}

impl TryFrom<String> for CarPoolStatus {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "APPLYING" => Ok(Self::Applying),
            "APL_COMPLETE" => Ok(Self::AplComplete),
            "CANCEL" => Ok(Self::Cancel),
            "FINISHED" => Ok(Self::Finished),
            _ => Err(Error::ValidateError("invalid carpool status".to_string())),
        }
    }
}

impl<'a> FromSql<'a> for CarPoolStatus {
    fn from_sql(
        type_: &Type,
        raw: &'a [u8],
    ) -> Result<Self, Box<dyn std::error::Error + 'static + Sync + Send>> {
        let s: String = FromSql::from_sql(type_, raw)?;
        Ok(s.try_into().map_err(|_| {
            Box::new(Error::ValidateError(
                "invalid user_use_case status".to_string(),
            ))
        })?)
    }
    fn accepts(_type_: &Type) -> bool {
        true
    }
}

impl Display for CarPoolStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CarPoolStatus::Applying => write!(f, "APPLYING"),
            CarPoolStatus::AplComplete => write!(f, "APL_COMPLETE"),
            CarPoolStatus::Cancel => write!(f, "CANCEL"),
            CarPoolStatus::Finished => write!(f, "FINISHED"),
        }
    }
}

impl ToSql for CarPoolStatus {
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

impl<'de> Deserialize<'de> for CarPoolStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        value
            .try_into()
            .map_err(|_| D::Error::custom("validate carpool status error"))
    }
}
