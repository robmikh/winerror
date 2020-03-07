extern crate serde;
extern crate bincode;

use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use winerror_core::*;

fn main() -> Result<(), Box<dyn Error>> {
    let file_names = vec![
        "hresults.txt",
        "ntstatus.txt",
        "win32err.txt",
        "dxgi.txt",
        "d2d.txt",
        "d3d11.txt",
        "wincodec.txt",
        "com.txt"
    ];
    for file_name in &file_names {
        println!("cargo:rerun-if-changed=data/{}", &file_name);
    }
    std::fs::create_dir_all("data/generated")?;

    generate_map_bincode(&file_names)?;

    Ok(())
}

fn generate_map_bincode(file_names: &[&str]) -> Result<(), Box<dyn Error>> {
    let map = create_map(file_names)?;
    let encoded: Vec<u8> = bincode::serialize(&map)?;
    let path = generated_dir()?.join("map_bincode.in");
    let mut file = BufWriter::new(File::create(&path)?);
    file.write_all(&encoded)?;

    Ok(())
}

fn create_map(file_names: &[&str]) -> Result<HashMap<i32, Vec<ErrorInfo>>, Box<dyn Error>> {
    let mut result: HashMap<i32, Vec<ErrorInfo>> = HashMap::new();

    for file_name in file_names{
        let input_file = File::open(format!("data/{}", &file_name))?;
        let reader = BufReader::new(input_file);

        for line in reader.lines() {
            let line = line?;
            let pieces: Vec<_> = line.splitn(3, ",").collect();
            let code = {
                let result = parse_code(&pieces[0]);
                match result {
                    Ok(code) => code,
                    Err(_) => panic!("Parse error! \"{}\" in ({})", &pieces[0], line),
                }
            };
            let name = pieces[1].replace("\"", "");
            let mut description = pieces[2].trim().to_string();
            let last_char = description.pop();
            assert_eq!(Some(','), last_char);

            let info = ErrorInfo {
                code: code,
                name: name.trim().to_string(),
                description: description,
            };

            if let Some(infos) = result.get_mut(&code) {
                infos.push(info);
            } else {
                result.insert(code, vec![info]);
            }
        }
    }

    let s_ok_info = ErrorInfo { 
        code: 0, 
        name: "S_OK".to_string(), 
        description: "\"Success.\"".to_string() 
    };
    if let Some(infos) = result.get_mut(&0) {
        infos.push(s_ok_info);
    } else {
        result.insert(0, vec![s_ok_info]);
    }

    Ok(result)
}

fn generated_dir() -> Result<std::path::PathBuf, Box<dyn Error>> {
    Ok(std::path::PathBuf::from(env::var("CARGO_MANIFEST_DIR")?).join("data").join("generated"))
}