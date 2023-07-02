use std::fs::File;
use std::io::Error;
use std::process::{Command, Stdio};

fn main() -> Result<(), Error> {
    let outputs = File::create("/tmp/out.txt")?;
    let errors = outputs.try_clone()?;

    Command::new("ls")
        .args(&["/tmp", "/tmp/oops"])
        .stdout(Stdio::from(outputs))
        .stderr(Stdio::from(errors))
        .spawn()?
        .wait_with_output()?;

    Ok(())
}
