use crate::domain::Domain;

#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq)]
pub struct ID {
    inner: i64,
}

impl Domain for ID {
    fn new(id: &str) -> Result<Self, String>
    where
        Self: Sized,
    {
    }

    fn to_string(self) -> String {
        todo!()
    }
}
