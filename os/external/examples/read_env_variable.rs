use std::env;
use std::fs;
use std::io::Error;

fn main() -> Result<(), Error> {

    // Create the contents of the default config file
    fs::write("/tmp/config-xyz", b"hello there!")?;

    // read `config_path` from the environment variable `CONFIG`.
    // If `CONFIG` isn't set, fall back to a default config path.
    let config_path = env::var("CONFIG_XYZ")
        .unwrap_or("/tmp/config-xyz".to_string());

    let config: String = fs::read_to_string(config_path)?;
    println!("Config file contents: {}", config);

    Ok(())
}
