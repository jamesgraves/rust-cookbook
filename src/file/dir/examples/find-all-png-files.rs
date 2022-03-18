use std::error::Error;
use glob::glob;

fn main() -> Result<(), Box<dyn Error>> {
    for entry in glob("**/*.png")? {
        println!("{}", entry?.display());
    }

    Ok(())
}
