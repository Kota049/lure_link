use super::*;

#[test]
fn valid() {
    let budget = 1000.to_string();
    let result = Budget::new(&budget);
    assert_eq!(result, Ok(Budget { inner: 1000 }));
}
