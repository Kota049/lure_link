use crate::error::Error;
use chrono::{DateTime, TimeZone, Utc};
use chrono_tz::Asia::Tokyo;
use chrono_tz::Tz;
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt::{Display, Formatter};
use tokio_postgres::types::private::BytesMut;
use tokio_postgres::types::{FromSql, IsNull, ToSql, Type};

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, Serialize, PartialOrd, PartialEq)]
pub struct JaTimeStamp(pub(crate) DateTime<Tz>);

impl Default for JaTimeStamp {
    fn default() -> Self {
        Utc.with_ymd_and_hms(2100, 1, 1, 0, 0, 0)
            .unwrap()
            .try_into()
            .unwrap()
    }
}

impl TryFrom<DateTime<Utc>> for JaTimeStamp {
    type Error = Error;
    fn try_from(value: DateTime<Utc>) -> Result<Self, Self::Error> {
        Ok(JaTimeStamp(value.with_timezone(&Tokyo)))
    }
}

impl<'a> FromSql<'a> for JaTimeStamp {
    fn from_sql(
        type_: &Type,
        raw: &'a [u8],
    ) -> Result<Self, Box<dyn std::error::Error + 'static + Sync + Send>> {
        let s: DateTime<Utc> = FromSql::from_sql(type_, raw)?;
        Ok(s.try_into()
            .map_err(|_| Box::new(Error::ValidateError("invalid timestamp".to_string())))?)
    }
    fn accepts(_type_: &Type) -> bool {
        true
    }
}

impl Display for JaTimeStamp {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ToSql for JaTimeStamp {
    fn to_sql(
        &self,
        _ty: &Type,
        out: &mut BytesMut,
    ) -> Result<IsNull, Box<dyn std::error::Error + 'static + Sync + Send>> {
        let timestamp = self.0.format("%Y-%m-%d %H:%M:%S%:z").to_string();
        out.extend_from_slice(timestamp.as_bytes());
        Ok(IsNull::No)
    }
    fn accepts(_ty: &Type) -> bool {
        true
    }
    tokio_postgres::types::to_sql_checked!();
}

impl<'de> Deserialize<'de> for JaTimeStamp {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let utc: DateTime<Utc> = DateTime::deserialize(deserializer)?;
        Ok(JaTimeStamp(utc.with_timezone(&Tokyo)))
    }
}
