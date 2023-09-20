use std::env;
use std::fs;

fn main() -> Result<(), std::io::Error> {
    let current_dir = env::current_dir()?;
    for entry in fs::read_dir(current_dir)? {
        let entry = entry?;
        println!("{}", entry.file_name().to_string_lossy());
    }
    Ok(())
}
