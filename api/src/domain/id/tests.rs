use super::*;

#[test]
fn valid() {
    let id = "11111";
    let result = ID::new(id);
    assert_eq!(result, Ok(ID { inner: 11111 }));
}
