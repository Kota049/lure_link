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
    assert_eq!(result, Err(INVALID_BUDGET_MESSAGE.to_string()));
}
#[test]
fn out_of_range() {
    let out_of_range = 100001.to_string();
    let result = Budget::new(&out_of_range);
    assert_eq!(result, Err(INVALID_BUDGET_MESSAGE.to_string()));
}

#[test]
fn to_string() {
    let budget = Budget { inner: 1000 };
    let result = budget.to_string();
    assert_eq!(result, "1000");
}
