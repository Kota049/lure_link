use crate::error::Error;
use crate::service::validation;
use crate::use_case::user_use_case::dto::UpdateUser;
#[cfg(test)]
mod tests;

pub fn validate_first_register_user(u: &UpdateUser) -> Result<(), Error> {
    if u.first_name.is_none() {
        return Err(Error::ValidateError(validation::not_register_message(
            "名前",
        )));
    }
    if u.first_name_jp.is_none() {
        return Err(Error::ValidateError(validation::not_register_message(
            "名前(フリガナ)",
        )));
    }
    if u.last_name.is_none() {
        return Err(Error::ValidateError(validation::not_register_message(
            "名字",
        )));
    }
    if u.last_name_jp.is_none() {
        return Err(Error::ValidateError(validation::not_register_message(
            "名字(フリガナ)",
        )));
    }
    if u.nick_name.is_none() {
        return Err(Error::ValidateError(validation::not_register_message(
            "ニックネーム",
        )));
    }
    Ok(())
}
