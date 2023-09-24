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
