use crate::constants::error_message::INVALID_POINT_NAME;
use crate::domain::Domain;
use crate::utils::validate::safe_string::validate_safe_string;

#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq)]
pub struct PointName {
    inner: String,
}

impl Domain for PointName {
    fn new(poin_name: &str) -> Result<Self, String>
    where
        Self: Sized,
    {
        validate_safe_string(poin_name).map_err(|_| INVALID_POINT_NAME.to_string())?;
        Ok(PointName {
            inner: poin_name.to_string(),
        })
    }

    fn to_string(self) -> String {
        self.inner
    }
}
