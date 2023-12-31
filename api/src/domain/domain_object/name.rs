use std::fmt::{Display, Formatter};
use serde::{Deserialize, Deserializer, Serialize};
use serde::de::Error as DeError;
use tokio_postgres::types::{FromSql, IsNull, ToSql, Type};
use tokio_postgres::types::private::BytesMut;
use crate::constants::regex::NAME_RULE;
use crate::db::connection::DbManager;
use crate::error::Error;

#[derive(Debug, Clone, Serialize, PartialOrd, PartialEq)]
pub struct Name(String);

impl TryFrom<String> for Name {
    type Error = Error;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if  !NAME_RULE.is_match(&value){
            return Err(Self::Error::ValidateError(format!("validation error name rule at {value}")));
        }
        Ok(Name(value))
    }
}

impl<'a> FromSql<'a> for Name {
    fn from_sql(type_: &Type, raw: &'a [u8]) -> Result<Self, Box<dyn std::error::Error + 'static + Sync + Send>> {
        let s: String = FromSql::from_sql(type_, raw)?;
        Ok(s.try_into().map_err(|_| Box::new(Error::ValidateError("invalid name".to_string())))?)
    }
    fn accepts(_type_: &Type) -> bool { true }
}

impl Display for Name {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",self.0)
    }
}

impl ToSql for Name {
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

impl<'de> Deserialize<'de> for Name {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        value.try_into().map_err(|_| D::Error::custom("validate name error"))
    }
}

#[tokio::test]
async fn test_raw_mapper() {
    let db = DbManager::new();
    // get test
    let row = db.client().await.query_one("SELECT '田中太郎', 'some'", &[]).await.unwrap();
    let name = row.try_get::<_, Name>(0).unwrap();
    assert_eq!(name, Name("田中太郎".to_string()));

    // set test
    let name = Name("田中太郎".to_string());
    let row = db.client().await.query_one("SELECT $1::TEXT AS name", &[&name]).await.unwrap();
    let res: Name = row.try_get("name").unwrap();
    assert_eq!(name, res);

    // couldn't get invalid name
    let row = db.client().await.query_one("SELECT ''", &[]).await.unwrap();
    let prefecture = row.try_get::<_, Name>(0);
    assert!(prefecture.is_err());

    // deserialize test
    let str = r#"
    {
        "test": "田中太郎"
    }
    "#;
    #[derive(Deserialize)]
    struct Test {
        pub test: Name,
    }
    let test: Test = serde_json::from_str(str).unwrap();
    assert_eq!(test.test, Name("田中太郎".to_string()));

    // couldn't deserialize invalid id
    let str = r#"
    {
        "test":""
    }
    "#;
    let res = serde_json::from_str::<Test>(str);
    assert!(res.is_err())
}