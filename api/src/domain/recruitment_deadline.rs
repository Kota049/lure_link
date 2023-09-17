use crate::utils::validate::date::validate_date;

#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq)]
pub struct RecruitmentDeadline {
    inner: String,
}

impl RecruitmentDeadline {
    pub fn new(deadline: &str) -> Result<Self, String> {
        if validate_date(deadline).is_err() {
            return Err("不正なrecruitment_deadlineです".to_string());
        }
        Ok(RecruitmentDeadline {
            inner: deadline.to_string(),
        })
    }
}
