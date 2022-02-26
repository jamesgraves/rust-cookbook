use std::process::Command;
use std::fs::{File, remove_file};
use std::{thread, time};

#[allow(dead_code)]

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


pub fn touch_file(pathname: &str) {
    File::create(pathname).unwrap();
}

pub fn look_for_file(pathname: &str, wait_limit: usize) {
    for _ in 0 .. wait_limit {
        if let Ok(_) = File::open(pathname) {
            remove_file(pathname).unwrap();
            return;
        }
        thread::sleep(time::Duration::from_millis(100));
    }
    panic!("waited too long for file: {pathname}");
}

#[cfg(test)]
mod tests {
    /*
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
    */
}

