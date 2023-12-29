use crate::constants::error_message::VALIDATE_DEADLINE_MESSAGE;
use crate::domain::Domain;
use crate::utils::validate::date::validate_date;

#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq)]
pub struct RecruitmentDeadline {
    inner: String,
}
impl Domain for RecruitmentDeadline {
    fn new(deadline: &str) -> Result<Self, String> {
        validate_date(deadline).map_err(|_| VALIDATE_DEADLINE_MESSAGE)?;
        Ok(RecruitmentDeadline {
            inner: deadline.to_string(),
        })
    }

    fn to_string(self) -> String {
        self.inner
    }
}