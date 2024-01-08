use serde::{Deserialize, Deserializer, Serialize};
use serde::de::Error as DeError;
use tokio_postgres::types::{FromSql, IsNull, ToSql, Type};
use tokio_postgres::types::private::BytesMut;
use crate::error::Error;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, Serialize, PartialOrd, PartialEq)]
pub struct Id(i64);

impl Id {
    pub fn validate(i: i64) -> bool {
        i > 0
    }
}

impl TryFrom<i64> for Id {
    type Error = Error;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        if !Id::validate(value) {
            return Err(Error::ValidateError("id is negative".to_string()));
        }
        Ok(Id(value))
    }
}

impl<'a> FromSql<'a> for Id {
    fn from_sql(type_: &Type, raw: &'a [u8]) -> Result<Self, Box<dyn std::error::Error + 'static + Sync + Send>> {
        let id: i64 = FromSql::from_sql(type_, raw)?;
        if !Id::validate(id) {
            return Err(Box::new(Error::ValidateError("id is negative".to_string())));
        }
        Ok(Id(id))
    }
    fn accepts(_type_: &Type) -> bool { true }
}

impl ToSql for Id {
    fn to_sql(&self, _ty: &Type, out: &mut BytesMut) -> Result<IsNull, Box<dyn std::error::Error + 'static + Sync + Send>> {
        let id = self.0;
        out.extend_from_slice(id.to_string().as_bytes());
        Ok(IsNull::No)
    }
    fn accepts(_type_: &Type) -> bool {
        true
    }
    tokio_postgres::types::to_sql_checked!();
}

impl<'de> Deserialize<'de> for Id {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let value = i64::deserialize(deserializer)?;
        if !Id::validate(value) {
            return Err(DeError::custom("id must be positive"));
        }
        Ok(Id(value))
    }
}
