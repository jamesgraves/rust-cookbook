use std::error::Error;
use glob::{glob_with, MatchOptions};


fn main() -> Result<(), Box<dyn Error>> {
    let options = MatchOptions {
        case_sensitive: false,
        ..Default::default()
    };

    for entry in glob_with("test_data/foo_[0-9]*.txt", options)? {
        println!("{}", entry?.display());
    }

    Ok(())
}
