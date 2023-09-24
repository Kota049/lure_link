use super::*;

#[test]
fn valid() {
    let prefecture = "埼玉県";
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

#[test]
fn to_string() {
    let prefecture = Prefecture {
        inner: "埼玉県".to_string(),
    };
    let result = prefecture.to_string();

    assert_eq!(result, "埼玉県");
}
