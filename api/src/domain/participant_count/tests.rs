use super::*;
#[test]
fn valid() {
    let count = 3.to_string();
    let result = ParticipantCount::new(&count);
    assert_eq!(result, Ok(ParticipantCount { inner: 3 }))
}
