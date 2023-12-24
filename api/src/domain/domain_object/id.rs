use serde::{Deserialize, Deserializer, Serialize};
use serde::de::Error as DeError;
use tokio_postgres::types::{FromSql, IsNull, ToSql, Type};
use tokio_postgres::types::private::BytesMut;
use crate::db::connection::DbManager;
use crate::error::Error;

#[derive(Debug, Clone, Serialize, PartialOrd, PartialEq)]
pub struct Id(i64);

impl Id {
    pub fn validate(i: i64) -> bool {
        i > 0
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

    // couldn't get negative id
    let row = db.client().await.query_one("SELECT -42::BIGINT", &[]).await.unwrap();
    let id = row.try_get::<_, Id>(0);
    assert!(id.is_err());

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

    // couldn't deserialize invalid id
    let str = r#"
    {
        "id":-42
    }
    "#;
    let id = serde_json::from_str::<Test>(str);
    assert!(id.is_err())
}