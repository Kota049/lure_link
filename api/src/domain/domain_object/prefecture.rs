use std::fmt::{Display, Formatter};
use serde::{Deserialize, Deserializer, Serialize};
use serde::de::Error as DeError;
use tokio_postgres::types::{FromSql, IsNull, ToSql, Type};
use tokio_postgres::types::private::BytesMut;
use crate::db::connection::DbManager;
use crate::error::Error;

#[derive(Debug, Clone, Serialize, PartialOrd, PartialEq)]
pub enum Prefecture {
    Hokkaido,
    Aomori,
    Iwate,
    Miyagi,
    Akita,
    Yamagata,
    Fukushima,
    Ibaraki,
    Tochigi,
    Gunma,
    Saitama,
    Chiba,
    Tokyo,
    Kanagawa,
    Niigata,
    Toyama,
    Ishikawa,
    Fukui,
    Yamanashi,
    Nagano,
    Gifu,
    Shizuoka,
    Aichi,
    Mie,
    Shiga,
    Kyoto,
    Osaka,
    Hyogo,
    Nara,
    Wakayama,
    Tottori,
    Shimane,
    Okayama,
    Hiroshima,
    Yamaguchi,
    Tokushima,
    Kagawa,
    Ehime,
    Kochi,
    Fukuoka,
    Saga,
    Nagasaki,
    Kumamoto,
    Oita,
    Miyazaki,
    Kagoshima,
    Okinawa,
}

impl TryFrom<String> for Prefecture {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "北海道" => Ok(Self::Hokkaido),
            "青森県" => Ok(Self::Aomori),
            "岩手県" => Ok(Self::Iwate),
            "宮城県" => Ok(Self::Miyagi),
            "秋田県" => Ok(Self::Akita),
            "山形県" => Ok(Self::Yamagata),
            "福島県" => Ok(Self::Fukushima),
            "茨城県" => Ok(Self::Ibaraki),
            "栃木県" => Ok(Self::Tochigi),
            "群馬県" => Ok(Self::Gunma),
            "埼玉県" => Ok(Self::Saitama),
            "千葉県" => Ok(Self::Chiba),
            "東京都" => Ok(Self::Tokyo),
            "神奈川県" => Ok(Self::Kanagawa),
            "新潟県" => Ok(Self::Niigata),
            "富山県" => Ok(Self::Toyama),
            "石川県" => Ok(Self::Ishikawa),
            "福井県" => Ok(Self::Fukui),
            "山梨県" => Ok(Self::Yamanashi),
            "長野県" => Ok(Self::Nagano),
            "岐阜県" => Ok(Self::Gifu),
            "静岡県" => Ok(Self::Shizuoka),
            "愛知県" => Ok(Self::Aichi),
            "三重県" => Ok(Self::Mie),
            "滋賀県" => Ok(Self::Shiga),
            "京都府" => Ok(Self::Kyoto),
            "大阪府" => Ok(Self::Osaka),
            "兵庫県" => Ok(Self::Hyogo),
            "奈良県" => Ok(Self::Nara),
            "和歌山県" => Ok(Self::Wakayama),
            "鳥取県" => Ok(Self::Tottori),
            "島根県" => Ok(Self::Shimane),
            "岡山県" => Ok(Self::Okayama),
            "広島県" => Ok(Self::Hiroshima),
            "山口県" => Ok(Self::Yamaguchi),
            "徳島県" => Ok(Self::Tokushima),
            "香川県" => Ok(Self::Kagawa),
            "愛媛県" => Ok(Self::Ehime),
            "高知県" => Ok(Self::Kochi),
            "福岡県" => Ok(Self::Fukuoka),
            "佐賀県" => Ok(Self::Saga),
            "長崎県" => Ok(Self::Nagasaki),
            "熊本県" => Ok(Self::Kumamoto),
            "大分県" => Ok(Self::Oita),
            "宮崎県" => Ok(Self::Miyazaki),
            "鹿児島県" => Ok(Self::Kagoshima),
            "沖縄県" => Ok(Self::Okinawa),
            _ => Err(Error::ValidateError("invalid prefecture name".to_string()))
        }
    }
}

impl<'a> FromSql<'a> for Prefecture {
    fn from_sql(type_: &Type, raw: &'a [u8]) -> Result<Self, Box<dyn std::error::Error + 'static + Sync + Send>> {
        let s: String = FromSql::from_sql(type_, raw)?;
        Ok(s.try_into().map_err(|_| Box::new(Error::ValidateError("invalid prefecture".to_string())))?)
    }
    fn accepts(_type_: &Type) -> bool { true }
}

impl Display for Prefecture {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Prefecture::Hokkaido => write!(f, "北海道"),
            Prefecture::Aomori => write!(f, "青森県"),
            Prefecture::Iwate => write!(f, "岩手県"),
            Prefecture::Miyagi => write!(f, "宮城県"),
            Prefecture::Akita => write!(f, "秋田県"),
            Prefecture::Yamagata => write!(f, "山形県"),
            Prefecture::Fukushima => write!(f, "福島県"),
            Prefecture::Ibaraki => write!(f, "茨城県"),
            Prefecture::Tochigi => write!(f, "栃木県"),
            Prefecture::Gunma => write!(f, "群馬県"),
            Prefecture::Saitama => write!(f, "埼玉県"),
            Prefecture::Chiba => write!(f, "千葉県"),
            Prefecture::Tokyo => write!(f, "東京都"),
            Prefecture::Kanagawa => write!(f, "神奈川県"),
            Prefecture::Niigata => write!(f, "新潟県"),
            Prefecture::Toyama => write!(f, "富山県"),
            Prefecture::Ishikawa => write!(f, "石川県"),
            Prefecture::Fukui => write!(f, "福井県"),
            Prefecture::Yamanashi => write!(f, "山梨県"),
            Prefecture::Nagano => write!(f, "長野県"),
            Prefecture::Gifu => write!(f, "岐阜県"),
            Prefecture::Shizuoka => write!(f, "静岡県"),
            Prefecture::Aichi => write!(f, "愛知県"),
            Prefecture::Mie => write!(f, "三重県"),
            Prefecture::Shiga => write!(f, "滋賀県"),
            Prefecture::Kyoto => write!(f, "京都府"),
            Prefecture::Osaka => write!(f, "大阪府"),
            Prefecture::Hyogo => write!(f, "兵庫県"),
            Prefecture::Nara => write!(f, "奈良県"),
            Prefecture::Wakayama => write!(f, "和歌山県"),
            Prefecture::Tottori => write!(f, "鳥取県"),
            Prefecture::Shimane => write!(f, "島根県"),
            Prefecture::Okayama => write!(f, "岡山県"),
            Prefecture::Hiroshima => write!(f, "広島県"),
            Prefecture::Yamaguchi => write!(f, "山口県"),
            Prefecture::Tokushima => write!(f, "徳島県"),
            Prefecture::Kagawa => write!(f, "香川県"),
            Prefecture::Ehime => write!(f, "愛媛県"),
            Prefecture::Kochi => write!(f, "高知県"),
            Prefecture::Fukuoka => write!(f, "福岡県"),
            Prefecture::Saga => write!(f, "佐賀県"),
            Prefecture::Nagasaki => write!(f, "長崎県"),
            Prefecture::Kumamoto => write!(f, "熊本県"),
            Prefecture::Oita => write!(f, "大分県"),
            Prefecture::Miyazaki => write!(f, "宮崎県"),
            Prefecture::Kagoshima => write!(f, "鹿児島県"),
            Prefecture::Okinawa => write!(f, "沖縄県"),
        }
    }
}

impl ToSql for Prefecture {
    fn to_sql(&self, _ty: &Type, out: &mut BytesMut) -> Result<IsNull, Box<dyn std::error::Error + 'static + Sync + Send>> {
        let prefecture = self.to_string();
        out.extend_from_slice(prefecture.as_bytes());
        Ok(IsNull::No)
    }
    fn accepts(_type_: &Type) -> bool {
        true
    }
    tokio_postgres::types::to_sql_checked!();
}

impl<'de> Deserialize<'de> for Prefecture {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        value.try_into().map_err(|_| D::Error::custom("validate prefecture error"))
    }
}

#[tokio::test]
async fn test_raw_mapper() {
    let db = DbManager::new();
    // get test
    let row = db.client().await.query_one("SELECT '埼玉県', 'some'", &[]).await.unwrap();
    let prefecture = row.try_get::<_, Prefecture>(0).unwrap();
    assert_eq!(prefecture, Prefecture::Saitama);

    // set test
    let prefecture = Prefecture::Saitama;
    let row = db.client().await.query_one("SELECT $1::TEXT AS prefecture", &[&prefecture]).await.unwrap();
    let res: Prefecture = row.try_get("prefecture").unwrap();
    assert_eq!(prefecture, res);

    // couldn't get invalid prefecture name
    let row = db.client().await.query_one("SELECT 'invalid'", &[]).await.unwrap();
    let prefecture = row.try_get::<_, Prefecture>(0);
    assert!(prefecture.is_err());

    // deserialize test
    let str = r#"
    {
        "test": "埼玉県"
    }
    "#;
    #[derive(Deserialize)]
    struct Test {
        pub test: Prefecture,
    }
    let test: Test = serde_json::from_str(str).unwrap();
    assert_eq!(test.test, Prefecture::Saitama);

    // couldn't deserialize invalid id
    let str = r#"
    {
        "test":"invalid"
    }
    "#;
    let res = serde_json::from_str::<Test>(str);
    assert!(res.is_err())
}