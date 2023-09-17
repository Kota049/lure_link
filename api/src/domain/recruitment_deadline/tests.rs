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
    assert_eq!(result, Err("不正なrecruitment_deadlineです".to_string()));
}
