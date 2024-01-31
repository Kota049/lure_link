use crate::entity::proposal::Proposal;
use crate::error::Error;

#[cfg(test)]
mod tests;

pub fn has_applying(exists_proposal: Result<Proposal, Error>) -> Result<(), Error> {
    match exists_proposal {
        Ok(_) => Err(Error::Other("already applying".to_string())),
        Err(Error::NotFound(_)) => Ok(()),
        Err(e) => Err(e),
    }
}
