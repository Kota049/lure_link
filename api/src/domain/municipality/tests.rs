use super::*;

const VALID_MUNICIPALITY: &'static str = "川越市";

#[test]
fn valid() {
    let municipality = VALID_MUNICIPALITY;
    let result = Municipality::new(municipality);
    assert_eq!(
        municipality,
        Ok(Municipality {
            inner: VALID_MUNICIPALITY.to_string()
        })
    );
}
