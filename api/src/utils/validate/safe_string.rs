pub fn validate_safe_string(s: &str) -> Result<(), String> {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::utils::validate::safe_string::validate_safe_string;

    #[test]
    fn valid() {
        let valid = "a";
        let result = validate_safe_string(&valid);
        assert_eq!(result, Ok(()));
    }
}
