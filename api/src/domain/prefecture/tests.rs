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
    assert_eq!(result, Err("不正なprefectureです".to_string()));
}
