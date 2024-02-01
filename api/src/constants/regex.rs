use once_cell::sync::Lazy;
use regex::Regex;

pub static MAIL_RULE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"^(("[^"]+"|[-a-zA-Z0-9._]+)@([-a-zA-Z0-9.]+\.[a-zA-Z]{2,}))$"#).unwrap()
});
pub static NAME_RULE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"^[a-zA-Z0-9_\-.ぁ-んァ-ヶ一-龠]{1,20}$"#).unwrap());
pub static FIRST_NAME_RULE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"^[a-zA-Z0-9_\-.ぁ-んァ-ヶ一-龠]{1,20}$"#).unwrap());
pub static LAST_NAME_RULE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"^[a-zA-Z0-9_\-.ぁ-んァ-ヶ一-龠]{1,20}$"#).unwrap());
pub static JAPANESE_NAME_RULE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\p{Script=Katakana}{1,30}").unwrap());
