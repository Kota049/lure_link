use crate::domain::domain_object::ja_timestamp::JaTimeStamp;
use crate::error::Error;
use chrono::Utc;

pub fn get_ja_now() -> Result<JaTimeStamp, Error> {
    Utc::now()
        .try_into()
        .map_err(|_| Error::InternalError("internal server error".to_string()))
}
