extern crate csv;

use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() -> Result<(), Box<Error>> {
    let out_dir = env::var("OUT_DIR")?;
    let dest_path = Path::new(&out_dir).join("names.rs");
    let mut f = File::create(&dest_path)?;

    write_array(
        &mut f,
        "FIRSTNAME_FEMALE",
        "src/data/firstname_female.csv",
        false,
    )?;
    write_array(
        &mut f,
        "FIRSTNAME_MALE",
        "src/data/firstname_male.csv",
        false,
    )?;
    write_array(&mut f, "SURNAME", "src/data/surname.csv", true)?;

    Ok(())
}

fn write_array(
    file: &mut File,
    constant_name: &str,
    path: &str,
    is_surname: bool,
) -> Result<(), Box<Error>> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(path)?;

    write!(file, "const {}: ArrayType = &[\n", constant_name)?;
    let mut total_frequency: u32 = 0;
    let mut total_count: u16 = 0;
    for result in reader.records() {
        total_count += 1;
        let r = result?;
        let name = dbg!(&r[0]);
        let fixed_name = if is_surname {
            lower_case_surname(name)
        } else {
            String::from(name)
        };
        let frequency: u32 = r[1].parse()?;
        total_frequency += frequency;
        write!(file, "  ({},\"{}\"),\n", total_frequency, fixed_name)?;
    }
    write!(file, "];\n")?;
    write!(
        file,
        "const {}_LEN: usize = {};\n",
        constant_name, total_count
    )?;
    Ok(())
}

fn lower_case_surname(uppercase_surname: &str) -> String {
    let lowercase_surname = uppercase_surname.to_lowercase();

    match &lowercase_surname {
        x if x.starts_with("mac") => {
            let (_, x) = x.split_at(3);
            format!("Mac{}", capitalise(x))
        }
        x if x.starts_with("mc") => {
            let (_, x) = x.split_at(2);
            format!("Mc{}", capitalise(x))
        }
        x if x.starts_with("o'") => {
            let (_, x) = x.split_at(2);
            format!("O'{}", capitalise(x))
        }
        x => capitalise(x),
    }
}

fn capitalise(s: &str) -> String {
    let (start, end) = s.split_at(1);
    format!("{}{}", start.to_uppercase(), end)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Sadly, this test doesn't currently run as cargo doesn't run tests for build.rs files.
    #[test]
    fn lower_case_surname_test() {
        assert_eq!("Robertson", lower_case_surname("ROBERTSON"));
        assert_eq!("MacKenzie", lower_case_surname("MACKENZIE"));
        assert_eq!("McMillan", lower_case_surname("MCMILLAN"));
        assert_eq!("O'Neill", lower_case_surname("O'NEILL"));
    }

}
