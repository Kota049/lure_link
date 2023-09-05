#[cfg(test)]
pub mod tests;

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub struct UserName {
    value: String,
}

impl UserName {
    #[allow(dead_code)]
    pub fn new(s: &str) -> Result<UserName, String> {
        // 空文字の場合
        if s == "" {
            return Err("空文字です".to_string());
        }

        // 半角英数記号が含まれる場合
        use regex::Regex;
        let regex_half_width = Regex::new(r"^[ -~]*$").unwrap();
        
        if regex_half_width.is_match(s){
            return Err("半角の文字が含まれています".to_string());
        }

        Ok(UserName {
            value: s.to_string(),
        })
    }
}
