use super::*;
#[test]
fn valid() {
    let nick_name = "サンプル太郎";
    let result = NickName::new(nick_name);
    assert_eq!(
        result,
        Ok(NickName {
            inner: nick_name.to_string()
        })
    );
}

#[test]
fn empty() {
    let empty = "";
    let result = NickName::new(empty);
    assert_eq!(result, Err(INVALID_NICK_NAME_MESSAGE.to_string()));
}

#[test]
fn too_large() {
    let too_large = "a".repeat(21);
    let result = NickName::new(&too_large);
    assert_eq!(result, Err(INVALID_NICK_NAME_MESSAGE.to_string()));
}

#[test]
fn to_string() {
    let nick_name = NickName {
        inner: "サンプル太郎".to_string(),
    };
    let result = nick_name.to_string();
    assert_eq!(result, "サンプル太郎");
}
