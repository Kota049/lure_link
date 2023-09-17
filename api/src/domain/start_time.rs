use crate::utils::validate::date;

#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq)]
pub struct StartDate {
    inner: String,
}

impl StartDate {
    pub fn new(start_date: &str) -> Result<Self, String> {
        date::validate_date(start_date)?;
        Ok(StartDate {
            inner: start_date.to_string(),
        })
    }
    pub fn to_string(&self) -> String {
        self.inner.clone()
    }
}
