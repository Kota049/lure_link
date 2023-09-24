use crate::domain::Domain;

#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq)]
pub struct Municipality {
    inner: String,
}

impl Domain for Municipality {
    fn new(municipality: &str) -> Result<Self, String>
    where
        Self: Sized,
    {
        Ok(Municipality {
            inner: municipality.to_string(),
        })
    }

    fn to_string(self) -> String {
        todo!()
    }
}
