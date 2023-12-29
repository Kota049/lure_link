use super::*;
use crate::constants::error_message;

const VALID_DATETIME: &'static str = "2023-09-17 12:34:56";

#[test]
fn valid() {
    let date = VALID_DATETIME;
    let result = StartDate::new(date);
    assert_eq!(
        result,
        Ok(StartDate {
            inner: VALID_DATETIME.to_string()
        })
    );
}
#[test]
fn empty_date() {
    let date = "";
    let result = StartDate::new(date);
    assert_eq!(result, Err(error_message::START_DATE.to_string()));
}
#[test]
fn non_date_format() {
    let date = "202309-17 12:34:56";
    let result = StartDate::new(date);
    assert_eq!(result, Err(error_message::START_DATE.to_string()));
}
#[test]
fn non_date_format_another() {
    let date = "202309-16 12:34:56";
    let result = StartDate::new(date);
    assert_eq!(result, Err(error_message::START_DATE.to_string()));
}

#[test]
fn to_string() {
    let start_date = StartDate::new(VALID_DATETIME).unwrap();
    let result = start_date.to_string();
    assert_eq!(result, VALID_DATETIME);
}