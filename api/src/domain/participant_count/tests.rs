use super::*;
#[test]
fn valid() {
    let count = 3.to_string();
    let result = ParticipantCount::new(&count);
    assert_eq!(result, Ok(ParticipantCount { inner: 3 }))
}

#[test]
fn non_integer() {
    let non_integer = "a";
    let result = ParticipantCount::new(non_integer);
    assert_eq!(result, Err(INVALID_PARTICIPANT_COUNT_MESSAGE.to_string()));
}

#[test]
fn out_of_range() {
    let out_of_range = 50.to_string();
    let result = ParticipantCount::new(&out_of_range);
    assert_eq!(result, Err(INVALID_PARTICIPANT_COUNT_MESSAGE.to_string()));
}
