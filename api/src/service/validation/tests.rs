use crate::service::validation::validate_first_register_user;
use crate::use_case::user_use_case::dto::UpdateUser;

#[test]
fn test_validate_first_register_user() {
    // valid case
    let up = UpdateUser::default();
    let res = validate_first_register_user(&up);
    assert!(res.is_ok());

    // lack of nick_name
    let up = UpdateUser {
        nick_name: None,
        ..UpdateUser::default()
    };
    let res = validate_first_register_user(&up);
    assert!(res.is_err());
    // lack of first_name
    let up = UpdateUser {
        first_name: None,
        ..UpdateUser::default()
    };
    let res = validate_first_register_user(&up);
    assert!(res.is_err());
    // lack of last_name
    let up = UpdateUser {
        last_name: None,
        ..UpdateUser::default()
    };
    let res = validate_first_register_user(&up);
    assert!(res.is_err());
    // lack of last_name_jp
    let up = UpdateUser {
        last_name_jp: None,
        ..UpdateUser::default()
    };
    let res = validate_first_register_user(&up);
    assert!(res.is_err());
    // lack of first_name_jp
    let up = UpdateUser {
        first_name_jp: None,
        ..UpdateUser::default()
    };
    let res = validate_first_register_user(&up);
    assert!(res.is_err());
}
