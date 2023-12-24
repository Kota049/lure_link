use tokio_postgres::types::FromSql;
use crate::constants::error_message::INVALID_ID_MESSAGE;
use crate::domain::Domain;

#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq)]
pub struct ID {
    inner: i64,
}

impl Domain for ID {
    fn new(id: &str) -> Result<Self, String>
    where
        Self: Sized,
    {
        let id = id
            .parse::<i64>()
            .map_err(|_| INVALID_ID_MESSAGE.to_string())?;
        if id < 1 {
            return Err(INVALID_ID_MESSAGE.to_string());
        }
        Ok(ID { inner: id })
    }

    fn to_string(self) -> String {
        self.inner.to_string()
    }
}
