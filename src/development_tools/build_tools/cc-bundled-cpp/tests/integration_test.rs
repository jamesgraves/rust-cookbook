
#[cfg(test)]
mod tests {
    use std::process::Command;

    #[test]
    fn default() {
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
}
