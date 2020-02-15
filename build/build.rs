use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

fn main() -> Result<(), Box<dyn Error>> {
    let file_names = vec![
        "hresults.txt",
        "ntstatus.txt"
    ];
    for file_name in &file_names {
        println!("cargo:rerun-if-changed=data/{}", &file_name);
    }
    std::fs::create_dir_all("data/generated")?;

    generate_code_lookup(&file_names)?;

    Ok(())
}

fn generate_code_lookup(file_names: &[&str]) -> Result<(), Box<dyn Error>> {
    let path = generated_dir()?.join("code_lookup_generated.in");
    let mut file = BufWriter::new(File::create(&path)?);

    write!(&mut file, "code_lookup!(\ncode,\n")?;

    for file_name in file_names{
        let input_file = File::open(format!("data/{}", &file_name))?;
        let reader = BufReader::new(input_file);

        for line in reader.lines() {
            let line = line?;
            write!(&mut file, "{}\n", &line)?;
        }
    }

    write!(&mut file, "0x0, \"S_OK\", \"Success code.\"\n)")?;

    Ok(())
}

fn generated_dir() -> Result<std::path::PathBuf, Box<dyn Error>> {
    Ok(std::path::PathBuf::from(env::var("CARGO_MANIFEST_DIR")?).join("data").join("generated"))
}