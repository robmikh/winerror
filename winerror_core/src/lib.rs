use std::i32;
use std::i64;
use std::fmt;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct ErrorInfo {
    pub code: i32,
    pub name: String,
    pub description: String,
}

impl fmt::Debug for ErrorInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ErrorInfo {{ code: {:#010x}, name: \"{}\", description: \"{}\" }}", self.code, self.name, self.description)
    }
}

pub fn parse_code(input: &str) -> Result<i32, std::num::ParseIntError> {
    if input.starts_with("0x") {
        // We do this to get around overflow checks. Otherwise,
        // inputs like 0x80070005 would fail.
        Ok(i64::from_str_radix(input.trim_start_matches("0x"), 16)? as i32)
    } else {
        // We do this to get around underflow and overflow checks. Otherwise,
        // inputs like -2147942405 would fail.
        Ok(input.parse::<i64>()? as i32)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    #[allow(overflowing_literals)]
    fn negative_input_test() {
        let code = crate::parse_code("-2147942405");
        assert_eq!(Ok(0x7ff8fffb), code);
    }

    #[test]
    #[allow(overflowing_literals)]
    fn negative_input_test2() {
        let code = crate::parse_code("-2147221164");
        assert_eq!(Ok(0x80040154), code);
    }

    #[test]
    #[allow(overflowing_literals)]
    fn positive_input_test() {
        let code = crate::parse_code("2147942405");
        assert_eq!(Ok(0x80070005), code);
    }

    #[test]
    #[allow(overflowing_literals)]
    fn hex_input_test() {
        let code = crate::parse_code("0x80070005");
        assert_eq!(Ok(0x80070005), code);
    }
}