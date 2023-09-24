use crate::constants::error_message::INVALID_BUDGET_MESSAGE;
use crate::domain::Domain;

#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq)]
pub struct Budget {
    inner: i32,
}

impl Domain for Budget {
    fn new(budget: &str) -> Result<Self, String>
    where
        Self: Sized,
    {
        let budget = budget
            .parse::<i32>()
            .map_err(|_| INVALID_BUDGET_MESSAGE.to_string())?;
        if budget < 0 || budget > 100000 {
            return Err(INVALID_BUDGET_MESSAGE.to_string());
        }
        Ok(Budget { inner: budget })
    }

    fn to_string(self) -> String {
        todo!()
    }
}
