use super::*;

const VALID_DATETIME: &'static str = "2023-09-17 12:34:56";

#[test]
fn valid(){
    let date = VALID_DATETIME;
    let result = StartDate::new(date);
    assert_eq!(result,Ok(StartDate{inner: VALID_DATETIME.to_string()}));
}
