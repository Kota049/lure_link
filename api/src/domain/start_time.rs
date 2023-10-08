use crate::domain::Domain;
use crate::utils::validate::date;

#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq)]
pub struct StartDate {
    inner: String,
}

impl Domain for StartDate {
    fn new(start_date: &str) -> Result<Self, String>
    where
        Self: Sized,
    {
        date::validate_date(start_date)?;
        Ok(StartDate {
            inner: start_date.to_string(),
        })
    }

    fn to_string(self) -> String {
        self.inner
    }
}
