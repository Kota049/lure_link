pub fn validate_safe_string(s: &str) -> Result<(), String> {
    is_empty(s)?;
    if s.chars().count() > 255 {
        return Err("不正な文字列です".to_string());
    }
    Ok(())
}

fn is_empty(s: &str) -> Result<(), String> {
    if s.is_empty() {
        return Err("不正な文字列です".to_string());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid() {
        let valid = "a";
        let result = validate_safe_string(&valid);
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn empty() {
        let valid = "";
        let result = validate_safe_string(&valid);
        assert_eq!(result, Err("不正な文字列です".to_string()));
    }

    #[test]
    fn too_large() {
        let valid = "a".repeat(256);
        let result = validate_safe_string(&valid);
        assert_eq!(result, Err("不正な文字列です".to_string()));
    }
}
