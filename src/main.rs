struct ErrorInfo {
    code: i32,
    name: String,
    description: String,
}

#[allow(overflowing_literals)]
fn get_error_info(code: i32) -> Option<ErrorInfo> {
    match code {
        0x00030200 => Some(ErrorInfo {
            code: 0x00030200,
            name: "STG_S_CONVERTED".to_string(),
            description: "The underlying file was converted to compound file format.".to_string(),
        }),
        0x80070005 => Some(ErrorInfo {
            code: 0x80070005,
            name: "E_ACCESSDENIED".to_string(),
            description: "General access denied error.".to_string(),
        }),
        _ => None,
    }
}

#[allow(overflowing_literals)]
fn main() {
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
        if let Some(info) = get_error_info(code) {
            println!("Code: {:#010x}", info.code);
            println!("Name: {}", info.name);
            println!("Description: {}", info.description);
        } else {
            println!("Not found!");
        }
    }
}
