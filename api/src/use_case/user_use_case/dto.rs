use crate::domain::domain_object::application_token::ApplicationToken;
use crate::domain::domain_object::first_name::FirstName;
use crate::domain::domain_object::last_name::LastName;
use crate::domain::domain_object::name::Name;
use crate::domain::domain_object::name_jp::NameJp;

pub struct UpdateUser {
    pub application_token: ApplicationToken,
    pub nick_name: Option<Name>,
    pub first_name: Option<FirstName>,
    pub last_name: Option<LastName>,
    pub first_name_jp: Option<NameJp>,
    pub last_name_jp: Option<NameJp>,
}

#[cfg(test)]
impl Default for UpdateUser {
    fn default() -> Self {
        UpdateUser {
            application_token: "a".to_string().try_into().unwrap(),
            nick_name: "some".to_string().try_into().ok(),
            first_name: "some".to_string().try_into().ok(),
            last_name: "some".to_string().try_into().ok(),
            first_name_jp: "カナ".to_string().try_into().ok(),
            last_name_jp: "カナ".to_string().try_into().ok(),
        }
    }
}
