use crate::entity::proposal::Proposal;
use crate::error::Error;
use crate::service::proposal_service::has_applying;

#[test]
fn test_has_applying() {
    let exists = Ok(Proposal::default());
    let res = has_applying(exists);
    assert!(res.is_err());

    let exists = Err(Error::Other("".to_string()));
    let res = has_applying(exists);
    assert!(res.is_err());

    let exists = Err(Error::NotFound("".to_string()));
    let res = has_applying(exists);
    assert!(res.is_ok());
}
