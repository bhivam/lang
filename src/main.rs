use std::fs;

use lang::scan_source;

fn main() -> Result<(), String> {
    let contents = match fs::read_to_string("data/source.lg") {
        Ok(contents) => contents,
        Err(error) => {
            return Err(format!("Failed to read data from file: {error}"));
        }
    };

    let tokens = scan_source(&contents);

    print!("{tokens:?}");

    Ok(())
}
