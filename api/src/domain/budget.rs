use crate::domain::Domain;

#[cfg(test)]
mod tests;

pub struct Budget {
    inner: i32,
}

impl Domain for Budget {
    fn new(_: &str) -> Result<Self, String>
    where
        Self: Sized,
    {
        todo!()
    }

    fn to_string(self) -> String {
        todo!()
    }
}
