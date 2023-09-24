use super::*;

#[test]
fn valid() {
    let id = "11111";
    let result = ID::new(id);
    assert_eq!(result, Ok(ID { inner: 11111 }));
}

#[test]
fn non_integer() {
    let non_integer = "a";
    let result = ID::new(non_integer);
    assert_eq!(result, Err(INVALID_ID_MESSAGE.to_string()));
}

#[test]
fn minus_integer() {
    let non_integer = "-1";
    let result = ID::new(non_integer);
    assert_eq!(result, Err(INVALID_ID_MESSAGE.to_string()));
}
