pub mod tests;

#[allow(dead_code)]
struct UserName {
    value: String,
}

impl UserName {
    #[allow(dead_code)]
    fn new(s: &str) -> Result<UserName, String> {
        if s == "" {
            return Err("空文字です".to_string());
        }

        Ok(UserName {
            value: s.to_string(),
        })
    }
}
