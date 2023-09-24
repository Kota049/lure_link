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
    assert_eq!(result, Err("不正なnick_nameです".to_string()));
}
