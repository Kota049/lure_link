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
        let budget = budget.parse::<i32>().unwrap();
        Ok(Budget { inner: budget })
    }

    fn to_string(self) -> String {
        todo!()
    }
}
