use super::*;

#[test]
fn valid() {
    let municipality = "川越市";
    let result = Municipality::new(municipality);
    assert_eq!(
        municipality,
        Ok(Municipality {
            inner: "川越市".to_string()
        })
    );
}
