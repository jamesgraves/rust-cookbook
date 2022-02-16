
#[cfg(test)]
mod tests {
    use std::process::Command;

    fn run_example(example_name: &str) {
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
            panic!("basic_usage failed and stderr was:\n{}", s);
        }
    }

    #[test]
    fn basic_usage() {
        run_example("basic_usage");
    }

    #[test]
    fn gen_range() {
        run_example("gen_range");
    }

    #[test]
    fn uniform_distribution() {
        run_example("uniform_distribution");
    }
}


