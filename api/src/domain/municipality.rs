use crate::constants::error_message::INVALID_MUNICIPALITY_MESSAGE;
use crate::domain::Domain;
use crate::utils::validate::safe_string::validate_safe_string;

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
        validate_safe_string(municipality).map_err(|_| INVALID_MUNICIPALITY_MESSAGE.to_string())?;
        Ok(Municipality {
            inner: municipality.to_string(),
        })
    }

    fn to_string(self) -> String {
        self.inner
    }
}
