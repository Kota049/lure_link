use crate::domain::Domain;

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
        Ok(NickName {
            inner: nick_name.to_string(),
        })
    }

    fn to_string(self) -> String {
        todo!()
    }
}
