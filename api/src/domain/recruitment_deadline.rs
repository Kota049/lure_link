#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq)]
pub struct RecruitmentDeadline {
    inner: String,
}

impl RecruitmentDeadline {
    pub fn new(deadline: &str) -> Result<Self, String> {
        todo!()
    }
}
