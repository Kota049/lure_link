use crate::constants::regex::JAPANESE_NAME_RULE;
use crate::error::Error;
use serde::de::Error as DeError;
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt::{Display, Formatter};
use tokio_postgres::types::private::BytesMut;
use tokio_postgres::types::{FromSql, IsNull, ToSql, Type};

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, Serialize, PartialOrd, PartialEq)]
pub struct NameJp(String);

impl TryFrom<String> for NameJp {
    type Error = Error;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if !JAPANESE_NAME_RULE.is_match(&value) {
            return Err(Error::ValidateError(format!(
                "validation error first name rule at {value}"
            )));
        }
        Ok(NameJp(value))
    }
}

impl<'a> FromSql<'a> for NameJp {
    fn from_sql(
        type_: &Type,
        raw: &'a [u8],
    ) -> Result<Self, Box<dyn std::error::Error + 'static + Sync + Send>> {
        let s: String = FromSql::from_sql(type_, raw)?;
        Ok(s.try_into()
            .map_err(|_| Box::new(Error::ValidateError("invalid name".to_string())))?)
    }
    fn accepts(_type_: &Type) -> bool {
        true
    }
}

impl Display for NameJp {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ToSql for NameJp {
    fn to_sql(
        &self,
        _ty: &Type,
        out: &mut BytesMut,
    ) -> Result<IsNull, Box<dyn std::error::Error + 'static + Sync + Send>> {
        let name = self.to_string();
        out.extend_from_slice(name.as_bytes());
        Ok(IsNull::No)
    }
    fn accepts(_type_: &Type) -> bool {
        true
    }
    tokio_postgres::types::to_sql_checked!();
}

impl<'de> Deserialize<'de> for NameJp {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        value
            .try_into()
            .map_err(|_| D::Error::custom("validate name error"))
    }
}
