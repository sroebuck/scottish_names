extern crate csv;

use std::env;
use std::fs::File;
use std::error::Error;
use std::io::Write;
use std::path::Path;

fn main() -> Result<(), Box<Error>> {
    let out_dir = env::var("OUT_DIR")?;
    let dest_path = Path::new(&out_dir).join("names.rs");
    let mut f = File::create(&dest_path)?;

    write_array(&mut f, "FIRSTNAME_FEMALE", "src/data/firstname_female.csv");
    write_array(&mut f, "FIRSTNAME_MALE", "src/data/firstname_male.csv");
    write_array(&mut f, "SURNAME", "src/data/surname.csv");

    Ok(())
}

fn write_array(file: &mut File, constant_name: &str, path: &str) -> Result<(), Box<Error>> {
    let mut reader = csv::ReaderBuilder::new().has_headers(false).from_path(path)?;

    write!(file, "static {}: &[(u32, &str)] = &[\n", constant_name)?;
    let mut total: u32 = 0;
    for result in reader.records() {
        let r = result?;
        let name = dbg!(&r[0]);
        let frequency: u32 = r[1].parse()?;
        total += frequency;
        write!(file, "  ({},\"{}\"),\n", total, name)?;
    }
    write!(file, "];\n")?;
    Ok(())
}