use super::*;
use serde_json::to_string;
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
    assert_eq!(result, Err("不正なparticipant_countです".to_string()));
}
