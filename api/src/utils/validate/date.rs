use crate::constants::error_message;

pub fn validate_date(s: &str) -> Result<(), String> {
    if s.is_empty() {
        return Err(error_message::START_DATE.to_string());
    }
    let mut s_char = s.chars();
    // 年の部分の確認
    for _ in 0..4 {
        if !s_char.next().is_some_and(|c| c.is_digit(10)) {
            return Err(error_message::START_DATE.to_string());
        }
    }
    // 区切り文字の確認
    if !s_char.next().is_some_and(|c| c == '-') {
        return Err(error_message::START_DATE.to_string());
    }
    // 月の部分の確認
    for _ in 0..2 {
        if !s_char.next().is_some_and(|c| c.is_digit(10)) {
            return Err(error_message::START_DATE.to_string());
        }
    }
    // 区切り文字の確認
    if !s_char.next().is_some_and(|c| c == '-') {
        return Err(error_message::START_DATE.to_string());
    }
    // 日にちの部分の確認
    for _ in 0..2 {
        if !s_char.next().is_some_and(|c| c.is_digit(10)) {
            return Err(error_message::START_DATE.to_string());
        }
    }
    // 区切り文字の確認
    if !s_char.next().is_some_and(|c| c == ' ') {
        return Err(error_message::START_DATE.to_string());
    }
    // 時間部分の確認
    for _ in 0..2 {
        if !s_char.next().is_some_and(|c| c.is_digit(10)) {
            return Err(error_message::START_DATE.to_string());
        }
    }
    // 区切り文字の確認
    if !s_char.next().is_some_and(|c| c == ':') {
        return Err(error_message::START_DATE.to_string());
    }
    // 分の部分の確認
    for _ in 0..2 {
        if !s_char.next().is_some_and(|c| c.is_digit(10)) {
            return Err(error_message::START_DATE.to_string());
        }
    }
    // 区切り文字の確認
    if !s_char.next().is_some_and(|c| c == ':') {
        return Err(error_message::START_DATE.to_string());
    }
    // 秒の部分の確認
    for _ in 0..2 {
        println!("{}", s_char.as_str());
        if !s_char.next().is_some_and(|c| c.is_digit(10)) {
            return Err(error_message::START_DATE.to_string());
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    macro_rules! date_test {
        ($test_name:ident, $input:expr, $expected:expr) => {
            #[test]
            fn $test_name() {
                let input = $input;
                let expected = $expected;
                let result = validate_date(input);
                assert_eq!(result, expected);
            }
        };
    }
    date_test!(
        non_disit_year,
        "202a-09-17 12:34:56",
        Err(error_message::START_DATE.to_string())
    );
    date_test!(
        non_hyphen_year,
        "2023a09-17 12:34:56",
        Err(error_message::START_DATE.to_string())
    );
    date_test!(
        non_disit_month,
        "2023-a9-17 12:34:56",
        Err(error_message::START_DATE.to_string())
    );
    date_test!(
        non_hyphen_month,
        "2023-09a17 12:34:56",
        Err(error_message::START_DATE.to_string())
    );
    date_test!(
        non_disit_date,
        "2023-09-1a 12:34:56",
        Err(error_message::START_DATE.to_string())
    );
    date_test!(
        non_space_date,
        "2023-09-17a12:34:56",
        Err(error_message::START_DATE.to_string())
    );
    date_test!(
        non_disit_hour,
        "2023-09-17 a2:34:56",
        Err(error_message::START_DATE.to_string())
    );
    date_test!(
        non_coron_hour,
        "2023-09-17 12a34:56",
        Err(error_message::START_DATE.to_string())
    );
    date_test!(
        non_disit_minute,
        "2023-09-17 12:a4:56",
        Err(error_message::START_DATE.to_string())
    );
    date_test!(
        non_coron_minute,
        "2023-09-17 12:34a56",
        Err(error_message::START_DATE.to_string())
    );
    date_test!(
        non_disit_second,
        "2023-09-17 12:34:a6",
        Err(error_message::START_DATE.to_string())
    );
    date_test!(valid, VALID_DATETIME, Ok(()));
    const VALID_DATETIME: &'static str = "2023-09-17 12:34:56";
}
