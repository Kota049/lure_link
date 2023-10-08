use super::*;
#[test]
fn valid() {
    let valid = "2023-09-17 12:34:56";
    let result = RecruitmentDeadline::new(valid).unwrap();
    assert_eq!(
        result,
        RecruitmentDeadline {
            inner: valid.to_string()
        }
    );
}
#[test]
fn invalid() {
    let valid = "2023-09-1a 12:34:56";
    let result = RecruitmentDeadline::new(valid);
    assert_eq!(result, Err(VALIDATE_DEADLINE_MESSAGE.to_string()));
}

#[test]
fn to_string() {
    let start_date = RecruitmentDeadline::new("2023-09-17 12:34:56").unwrap();
    let result = start_date.to_string();
    assert_eq!(result, "2023-09-17 12:34:56");
}
