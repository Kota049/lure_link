// only use Struct(String) , other use case occurs compile error
#[macro_export]
macro_rules! impl_text_rule {
    ($struct_name:ident, $max:expr) => {
        use std::fmt::{Display, Formatter};
        use serde::{Deserialize, Deserializer};
        use serde::de::Error as DeError;
        use tokio_postgres::types::{FromSql, IsNull, ToSql, Type};
        use tokio_postgres::types::private::BytesMut;
        use crate::error::Error;
        impl TryFrom<String> for $struct_name {
            type Error = Error;
            fn try_from(value: String) -> Result<Self, Self::Error> {
                if  value.len() < 1 || value.len() > $max {
                    return Err(Self::Error::ValidateError(format!("validation error text_domain rule at {value} : max length = {}",$max)));
                }
                Ok($struct_name(value))
            }
        }

        impl<'a> FromSql<'a> for $struct_name {
            fn from_sql(type_: &Type, raw: &'a [u8]) -> Result<Self, Box<dyn std::error::Error + 'static + Sync + Send>> {
                let s: String = FromSql::from_sql(type_, raw)?;
                Ok(s.try_into().map_err(|_| Box::new(Error::ValidateError("invalid name".to_string())))?)
            }
            fn accepts(_type_: &Type) -> bool { true }
        }

        impl Display for $struct_name {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                write!(f,"{}",self.0)
            }
        }

        impl ToSql for $struct_name {
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

        impl<'de> Deserialize<'de> for $struct_name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where
                    D: Deserializer<'de>,
            {
                let value = String::deserialize(deserializer)?;
                value.try_into().map_err(|_| D::Error::custom("validate name error"))
            }
        }
    };
}