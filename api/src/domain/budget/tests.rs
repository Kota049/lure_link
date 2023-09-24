use super::*;

#[test]
fn valid() {
    let budget = 1000.to_string();
    let result = Budget::new(&budget);
    assert_eq!(result, Ok(Budget { inner: 1000 }));
}

#[test]
fn non_integer() {
    let non_integer = "a";
    let result = Budget::new(non_integer);
    assert_eq!(result, Err("不正なbudgetです".to_string()));
}
