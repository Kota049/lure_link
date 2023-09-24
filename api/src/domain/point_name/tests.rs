use super::*;

#[test]
fn valid() {
    let prefecture = VALID_POINT_NAME;
    let result = PointName::new(prefecture);
    assert_eq!(
        result,
        Ok(PointName {
            inner: prefecture.to_string()
        })
    )
}

#[test]
fn invalid() {
    let prefecture = ";";
    let result = PointName::new(prefecture);
    assert_eq!(result, Err(INVALID_POINT_NAME.to_string()));
}

const VALID_POINT_NAME: &'static str = "川越駅前";

#[test]
fn to_string() {
    let prefecture = PointName {
        inner: VALID_POINT_NAME.to_string(),
    };
    let result = prefecture.to_string();

    assert_eq!(result, VALID_POINT_NAME);
}
