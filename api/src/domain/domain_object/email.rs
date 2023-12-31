use std::fmt::{Display, Formatter};
use serde::{Deserialize, Deserializer, Serialize};
use serde::de::Error as DeError;
use tokio_postgres::types::{FromSql, IsNull, ToSql, Type};
use tokio_postgres::types::private::BytesMut;
use crate::constants::regex::MAIL_RULE;
use crate::error::Error;
#[cfg(test)]
mod tests;
#[derive(Debug, Clone, Serialize, PartialOrd, PartialEq)]
pub struct MailAddress(String);

impl TryFrom<String> for MailAddress {
    type Error = Error;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if !MAIL_RULE.is_match(&value) {
            return Err(Self::Error::ValidateError(format!("validation error mail rule at {value}")));
        }
        Ok(MailAddress(value))
    }
}

impl<'a> FromSql<'a> for MailAddress {
    fn from_sql(type_: &Type, raw: &'a [u8]) -> Result<Self, Box<dyn std::error::Error + 'static + Sync + Send>> {
        let s: String = FromSql::from_sql(type_, raw)?;
        Ok(s.try_into().map_err(|_| Box::new(Error::ValidateError("invalid mail".to_string())))?)
    }
    fn accepts(_type_: &Type) -> bool { true }
}

impl Display for MailAddress {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ToSql for MailAddress {
    fn to_sql(&self, _ty: &Type, out: &mut BytesMut) -> Result<IsNull, Box<dyn std::error::Error + 'static + Sync + Send>> {
        let name = self.to_string();
        out.extend_from_slice(name.as_bytes());
        Ok(IsNull::No)
    }
    fn accepts(_type_: &Type) -> bool {
        true
    }
    tokio_postgres::types::to_sql_checked!();
}

impl<'de> Deserialize<'de> for MailAddress {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        value.try_into().map_err(|_| D::Error::custom("validate email error"))
    }
}