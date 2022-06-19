#[cfg(test)]
mod tests {
    use cookbook_utils::run_example;

    #[test]
    fn semver_command() {
        run_example("semver-command");
    }

    #[test]
    fn semver_complex() {
        run_example("semver-complex");
    }

    #[test]
    fn semver_latest() {
        run_example("semver-latest");
    }

    #[test]
    fn semver_prerelease() {
        run_example("semver-prerelease");
    }
}
