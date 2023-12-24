use serde::{de, Deserialize, Deserializer, Serialize};
use serde::de::Error as DeError;
use tokio_postgres::types::{FromSql, IsNull, ToSql, Type};
use tokio_postgres::types::private::BytesMut;
use crate::db::connection::DbManager;
use crate::error::Error;

#[derive(Debug, Clone, Serialize, PartialOrd, PartialEq)]
pub struct Id(i64);

impl<'a> FromSql<'a> for Id {
    fn from_sql(type_: &Type, raw: &'a [u8]) -> Result<Self, Box<dyn std::error::Error + 'static + Sync + Send>> {
        let id: i64 = FromSql::from_sql(type_, raw)?;
        if id < 0 {
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
    fn accepts(type_: &Type) -> bool {
        // この実装は全てのTypeに対応することを示しています
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
        if value < 0{
            return Err(DeError::custom("id must be positive"));
        }
        Ok(Id(value))
        // struct IdVisitor;
        //
        // impl<'de> de::Visitor<'de> for IdVisitor {
        //     type Value = Id;
        //
        //     fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        //         formatter.write_str("struct Id")
        //     }
        //
        //     fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
        //         where
        //             E: de::Error,
        //     {
        //         // i64が0以上の場合のみデシリアライズを許可
        //         if value >= 0 {
        //             Ok(Id(value))
        //         } else {
        //             Err(de::Error::custom("Id must be non-negative"))
        //         }
        //     }
        // }
        //
        // deserializer.deserialize_i64(IdVisitor)
    }
}

#[tokio::test]
async fn test_raw_mapper() {
    let db = DbManager::new();
    // get test
    let row = db.client().await.query_one("SELECT 100::BIGINT, 'some'", &[]).await.unwrap();
    let id: Id = row.try_get(0).unwrap();
    assert_eq!(id, Id(100));

    // set test
    let id = Id(42);
    let row = db.client().await.query_one("SELECT $1::TEXT::BIGINT AS id", &[&id]).await.unwrap();
    let id: Id = row.try_get("id").unwrap();
    assert_eq!(id, Id(42));

    // deserialize test
    let str = r#"
    {
        "id":42
    }
    "#;
    #[derive(Deserialize)]
    struct Test {
        pub id: Id,
    }
    let id: Test = serde_json::from_str(str).unwrap();
    assert_eq!(id.id, Id(42));
}