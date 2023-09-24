use super::*;

const VALID_MUNICIPALITY: &'static str = "川越市";

#[test]
fn valid() {
    let municipality = VALID_MUNICIPALITY;
    let result = Municipality::new(municipality);
    assert_eq!(
        result,
        Ok(Municipality {
            inner: VALID_MUNICIPALITY.to_string()
        })
    );
}

#[test]
fn invalid() {
    let unsafe_string = ";";
    let result = Municipality::new(unsafe_string);
    assert_eq!(result, Err(INVALID_MUNICIPALITY_MESSAGE.to_string()));
}
