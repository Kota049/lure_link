use crate::error::Error;
use serde::de::Error as DeError;
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt::{Display, Formatter};
use tokio_postgres::types::private::BytesMut;
use tokio_postgres::types::{FromSql, IsNull, ToSql, Type};

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, Serialize, PartialOrd, PartialEq)]
pub enum PaymentStatus {
    Free,
    Authorization,
    Cancel,
    SettleFixed,
}

impl TryFrom<String> for PaymentStatus {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "AUTHORIZATION" => Ok(Self::Authorization),
            "FREE" => Ok(Self::Free),
            "CANCEL" => Ok(Self::Cancel),
            "SETTLE_FIXED" => Ok(Self::SettleFixed),
            _ => Err(Error::ValidateError("invalid payment status".to_string())),
        }
    }
}

impl<'a> FromSql<'a> for PaymentStatus {
    fn from_sql(
        type_: &Type,
        raw: &'a [u8],
    ) -> Result<Self, Box<dyn std::error::Error + 'static + Sync + Send>> {
        let s: String = FromSql::from_sql(type_, raw)?;
        Ok(s.try_into()
            .map_err(|_| Box::new(Error::ValidateError("invalid payment status".to_string())))?)
    }
    fn accepts(_type_: &Type) -> bool {
        true
    }
}

impl Display for PaymentStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PaymentStatus::Authorization => write!(f, "AUTHORIZATION"),
            PaymentStatus::Free => write!(f, "FREE"),
            PaymentStatus::Cancel => write!(f, "CANCEL"),
            PaymentStatus::SettleFixed => write!(f, "SETTLE_FIXED"),
        }
    }
}

impl ToSql for PaymentStatus {
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

impl<'de> Deserialize<'de> for PaymentStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        value
            .try_into()
            .map_err(|_| D::Error::custom("validate payment status error"))
    }
}

#[cfg(test)]
impl Default for PaymentStatus {
    fn default() -> Self {
        Self::Free
    }
}
