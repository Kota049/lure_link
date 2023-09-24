use crate::domain::Domain;
use crate::utils::validate::safe_string::validate_safe_string;

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
        validate_safe_string(prefecture).map_err(|_| "不正なprefectureです".to_string())?;
        Ok(Prefecture {
            inner: prefecture.to_string(),
        })
    }

    fn to_string(self) -> String {
        todo!()
    }
}
