use crate::domain::Domain;

#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq)]
pub struct Prefecture {
    inner: String,
}

impl Domain for Prefecture {
    fn new(prefecture: &str) -> Result<Self, String>
    where
        Self: Sized,
    {
        Ok(Prefecture {
            inner: prefecture.to_string(),
        })
    }

    fn to_string(self) -> String {
        todo!()
    }
}
