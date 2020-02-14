use std::i32;
use std::i64;
use std::fmt;

struct ErrorInfo {
    code: i32,
    name: String,
    description: String,
}

impl fmt::Debug for ErrorInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ErrorInfo {{ code: {:#010x}, name: \"{}\", description: \"{}\" }}", self.code, self.name, self.description)
    }
}

macro_rules! hresults {
    ( $input:expr, $( $code:expr, $name:tt, $desc:tt ),* ) => {
        match $input {
            $(
                $code => Some(ErrorInfo {
                    code: $code,
                    name: $name.to_string(),
                    description: $desc.to_string(),
                }),
            )*
            _ => None,
        }
    }
}

#[allow(overflowing_literals)]
fn get_error_info_from_code(code: i32) -> Option<ErrorInfo> {
    include!("..\\data\\generated\\hresults_generated.in")
}

fn get_error_info_from_str(input: &str) -> Option<ErrorInfo> {
    let code = parse_code(input);
    get_error_info_from_code(code)
}

fn parse_code(input: &str) -> i32 {
    if input.starts_with("0x") {
        // We do this to get around overflow checks. Otherwise,
        // inputs like 0x80070005 would fail.
        i64::from_str_radix(input.trim_start_matches("0x"), 16).unwrap() as i32
    } else {
        // We do this to get around overflow checks. Otherwise,
        // inputs like -2147942405 would fail.
        let input = input.replace("-", "");
        input.parse::<i64>().unwrap() as i32
    }
}

fn main() {
    if let Some(input) = std::env::args().nth(1) {
        let input = input.trim();
        if let Some(info) = get_error_info_from_str(&input) {
            println!("{:#010x}", info.code);
            println!("{}", info.name);
            println!("{}", info.description);
        } else {
            println!("No matches found for \"{:#010x}\"!", parse_code(&input));
        }
    } else {
        // TODO: print usage
    }
}

#[cfg(test)]
mod tests {
    // TODO: Write a real test
    #[test]
    #[allow(overflowing_literals)]
    fn scratch() {
        let codes = vec![
            0x00030200,
            197120,
            0,
            88,
            0x80070005,
            -2147942405,
            2147942405,
        ];

        println!("Running test...");
        for code in codes {
            if let Some(info) = crate::get_error_info_from_code(code) {
                println!("Code: {:#010x}", info.code);
                println!("Name: {}", info.name);
                println!("Description: {}", info.description);
            } else {
                println!("Not found!");
            }
        }
    }
}