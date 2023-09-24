use crate::constants::error_message::INVALID_NICK_NAME_MESSAGE;
use crate::domain::Domain;
use crate::utils::validate::safe_string::validate_safe_string;

#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq)]
pub struct NickName {
    inner: String,
}

impl Domain for NickName {
    fn new(nick_name: &str) -> Result<Self, String>
    where
        Self: Sized,
    {
        if nick_name.chars().count() > 20 {
            return Err(INVALID_NICK_NAME_MESSAGE.to_string());
        }
        validate_safe_string(nick_name).map_err(|_| INVALID_NICK_NAME_MESSAGE.to_string())?;
        Ok(NickName {
            inner: nick_name.to_string(),
        })
    }

    fn to_string(self) -> String {
        self.inner
    }
}
