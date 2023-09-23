use crate::constants::error_message::INVALID_COMMON_MESSAGE;

pub fn validate_safe_string(s: &str) -> Result<(), String> {
    check_length(s)?;
    if s.contains('<') {
        return Err(INVALID_COMMON_MESSAGE.to_string());
    }
    Ok(())
}

fn check_length(s: &str) -> Result<(), String> {
    if s.is_empty() {
        return Err(INVALID_COMMON_MESSAGE.to_string());
    }
    if s.chars().count() > 100 {
        return Err(INVALID_COMMON_MESSAGE.to_string());
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
        assert_eq!(result, Err(INVALID_COMMON_MESSAGE.to_string()));
    }

    #[test]
    fn too_large() {
        let valid = "a".repeat(101);
        let result = validate_safe_string(&valid);
        assert_eq!(result, Err(INVALID_COMMON_MESSAGE.to_string()));
    }
    #[test]
    fn comparator() {
        let valid = "a<";
        let result = validate_safe_string(&valid);
        assert_eq!(result, Err(INVALID_COMMON_MESSAGE.to_string()));
    }
    #[test]
    fn opposite_comparator() {
        let valid = "a>";
        let result = validate_safe_string(&valid);
        assert_eq!(result, Err(INVALID_COMMON_MESSAGE.to_string()));
    }
}
