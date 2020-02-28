#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;
use std::i32;
use winerror_core::*;

lazy_static! {
    static ref CODE_MAP: HashMap<i32, Vec<ErrorInfo>> = {
        let map: HashMap<i32, Vec<ErrorInfo>> = bincode::deserialize(include_bytes!("..\\data\\generated\\map_bincode.in")).unwrap();
        map
    };
}

#[allow(overflowing_literals)]
fn get_error_infos(code: i32) -> Option<Vec<ErrorInfo>> {
    if let Some(infos) = CODE_MAP.get(&code) {
        Some(infos.to_vec())
    } else {
        None
    }
}

fn print_usage() {
    println!("Usage: winerror <code>");
    println!("Examples:");
    println!("winerror 0x80070005");
    println!("winerror 2147942405");
}

fn lookup_error_code(code: i32) -> bool {
    if let Some(infos) = get_error_infos(code) {
        println!("{} match(es) found:", infos.len());
        for info in infos {
            println!("{:#010x}", info.code);
            println!("    {}", info.name);
            println!("    {}", info.description);
        }
        true
    } else {
        println!("No matches found for \"{:#010x}\"!", code);
        false
    }
}

fn lookup_from_str(input: &str) -> bool {
    let input = input.trim();
    if let Ok(code) = parse_code(&input) {
        lookup_error_code(code)
    } else {
        println!("Invalid input!");
        print_usage();
        false
    }
}

fn main() {
    if let Some(input) = std::env::args().nth(1) {
        lookup_from_str(&input);
    } else {
        print_usage();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    #[allow(overflowing_literals)]
    fn some_codes_lookup_test() {
        let codes = vec![
            (0x00030200, true),
            (197120, true),
            (0, true),
            (0x80070000, false),
            (0x80070005, true),
            (-2147942405, false),
            (2147942405, true),
        ];
        
        let mut temp = 0;
        for (code, will_succeed) in codes {
            if let Some(infos) = crate::get_error_infos(code) {
                assert!(will_succeed);
                for info in infos {
                    assert_eq!(code, info.code);
                }
            } else {
                if will_succeed {
                    panic!("Unexpected lookup failure! {} ({:#010x}) was expected to succeed! {}", code, code, temp);
                }
            }
            temp = temp + 1;
        }
    }

    #[test]
    fn successfull_hex_lookup() {
        assert!(crate::lookup_from_str("0x80070005"));
    }

    #[test]
    fn successfull_dec_lookup() {
        assert!(crate::lookup_from_str("2147942405"));
    }

    #[test]
    fn successfull_neg_lookup() {
        assert!(crate::lookup_from_str("-2147221164"));
    }

    #[test]
    fn unsuccessfull_neg_lookup() {
        assert!(!crate::lookup_from_str("-2147942405"));
    }

    #[test]
    fn unsuccessfull_lookup() {
        assert!(!crate::lookup_from_str("0x80070000"));
    }
}