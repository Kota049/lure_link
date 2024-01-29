use crate::error::Error;
use crate::use_case::user_use_case::dto::UpdateUser;

#[cfg(test)]
mod tests;

pub fn validate_first_register_user(u: &UpdateUser) -> Result<(), Error> {
    if u.first_name.is_none() {
        return Err(Error::ValidateError("名前が登録されていません".to_string()));
    }
    if u.first_name_jp.is_none() {
        return Err(Error::ValidateError(
            "名前(フリガナ)が登録されていません".to_string(),
        ));
    }
    if u.last_name.is_none() {
        return Err(Error::ValidateError("名字が登録されていません".to_string()));
    }
    if u.last_name_jp.is_none() {
        return Err(Error::ValidateError(
            "名字(フリガナ)が登録されていません".to_string(),
        ));
    }
    if u.nick_name.is_none() {
        return Err(Error::ValidateError(
            "ニックネームが登録されていません".to_string(),
        ));
    }

    Ok(())
}
