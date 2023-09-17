#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq)]
pub struct RecruitmentDeadline {
    inner: String,
}

impl RecruitmentDeadline {
    pub fn new(deadline: &str) -> Result<Self, String> {
        Ok(RecruitmentDeadline {
            inner: deadline.to_string(),
        })
    }
}
