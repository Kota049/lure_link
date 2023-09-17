use crate::constants::*;

#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq)]
pub struct StartDate {
    inner: String,
}

impl StartDate {
    pub fn new(start_date: &str) -> Result<Self, String> {
        validate_date(start_date)?;
        Ok(StartDate {
            inner: start_date.to_string(),
        })
    }
}

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
