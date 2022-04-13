use std::process::Command;

#[allow(dead_code)]

pub fn run() {
    println!("running program");
    let output = Command::new("cargo")
        .arg("run")
        .output().unwrap_or_else(|e| {
            panic!("failed to execute process: {}", e)
        });

    if ! output.status.success() {
        let s = String::from_utf8_lossy(&output.stderr);
        panic!("failed and stderr was:\n{}", s);
    }
}


pub fn run_example(example_name: &str) {
    println!("running example: {example_name}");
    let output = Command::new("cargo")
        .arg("run")
        .arg("--example")
        .arg(example_name)
        .output().unwrap_or_else(|e| {
            panic!("failed to execute process: {}", e)
        });

    if ! output.status.success() {
        let s = String::from_utf8_lossy(&output.stderr);
        panic!("{example_name} failed and stderr was:\n{}", s);
    }
}
