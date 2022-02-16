
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
            panic!("{example_name} failed and stderr was:\n{}", s);
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

    #[test]
    fn generate_password() {
        run_example("generate_password");
    }

    #[test]
    fn normal_distribution() {
        run_example("normal_distribution");
    }

    #[test]
    fn random_string() {
        run_example("random_string");
    }

    #[test]
    fn tuple_distribution() {
        run_example("tuple_distribution");
    }
}


