use std::error::Error;
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::Path;
use flate2::Compression;
use flate2::write::GzEncoder;

// Write contents of string into a file
fn echo(contents: &str, path_name: &str) -> Result<(), Box<dyn Error>> {
    let file_path = Path::new(path_name);
    let mut file_ref = File::create(file_path)?;
    file_ref.write_all(contents.as_bytes())?;
    Ok(())
}

// create some files, and then archive them
fn main() -> Result<(), Box<dyn Error>> {
    // make two directories, it is OK if they already exist
    create_dir_all("temp1")?;
    create_dir_all("temp7")?;

    // make four files
    echo("first\n", "temp1/one.txt")?;
    echo("second\n", "temp1/two.txt")?;
    echo("third\n", "temp7/three.txt")?;
    echo("fourth\n", "temp7/four.txt")?;

    // create archive file using gzip compression
    let file_ref = File::create("archive.tar.gz")?;
    let encoder = GzEncoder::new(file_ref, Compression::default());
    let mut archive = tar::Builder::new(encoder);

    // Add directories and files to archive, renaming one of them inside
    // the archive itself.
    archive.append_dir_all("temp1-renamed", "temp1")?;
    archive.append_dir_all("temp7", "temp7")?;

    println!("successfully created archive");
    Ok(())
}
