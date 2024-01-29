use crate::error::Error;
use crate::use_case::user_use_case::dto::UpdateUser;

#[cfg(test)]
mod tests;

pub fn validate_first_register_user(u: &UpdateUser) -> Result<(), Error> {
    todo!()
}
