use super::*;

#[test]
fn valid() {
    let prefecture = VALID_PREFECTURE;
    let result = Prefecture::new(prefecture);
    assert_eq!(
        result,
        Ok(Prefecture {
            inner: prefecture.to_string()
        })
    )
}

#[test]
fn invalid() {
    let prefecture = ";";
    let result = Prefecture::new(prefecture);
    assert_eq!(result, Err(INVALID_PREFECTURE_MESSAGE.to_string()));
}

const VALID_PREFECTURE: &'static str = "埼玉県";

#[test]
fn to_string() {
    let prefecture = Prefecture {
        inner: VALID_PREFECTURE.to_string(),
    };
    let result = prefecture.to_string();

    assert_eq!(result, VALID_PREFECTURE);
}
