use crate::domain::Domain;

#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq)]
pub struct ParticipantCount {
    inner: i16,
}

impl Domain for ParticipantCount {
    fn new(_: &str) -> Result<Self, String>
    where
        Self: Sized,
    {
        todo!()
    }

    fn to_string(self) -> String {
        todo!()
    }
}
