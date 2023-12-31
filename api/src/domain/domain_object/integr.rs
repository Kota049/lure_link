// only use Struct(i32) , other use case occurs compile error
#[macro_export]
macro_rules! impl_integer_rule {
    ($struct_name:ident, $max:expr) => {
        use std::fmt::{Display, Formatter};
        use serde::{Deserialize, Deserializer};
        use serde::de::Error as DeError;
        use tokio_postgres::types::{FromSql, IsNull, ToSql, Type};
        use tokio_postgres::types::private::BytesMut;
        use crate::error::Error;
        impl TryFrom<i32> for $struct_name {
            type Error = Error;
            fn try_from(value: i32) -> Result<Self, Self::Error> {
                if value < 1  ||  value > $max {
                    return Err(Self::Error::ValidateError(format!("validation error integer rule at {value} : max  = {}",$max)));
                }
                Ok($struct_name(value))
            }
        }

        impl<'a> FromSql<'a> for $struct_name {
            fn from_sql(type_: &Type, raw: &'a [u8]) -> Result<Self, Box<dyn std::error::Error + 'static + Sync + Send>> {
                let s: i32 = FromSql::from_sql(type_, raw)?;
                Ok(s.try_into().map_err(|_| Box::new(Error::ValidateError("invalid integer".to_string())))?)
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
                let v = self.0;
                out.extend_from_slice(v.to_string().as_bytes());
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
                let value = i32::deserialize(deserializer)?;
                value.try_into().map_err(|_| D::Error::custom("validate integer error"))
            }
        }
    };
}