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
        todo!()
    }

    fn to_string(self) -> String {
        todo!()
    }
}
