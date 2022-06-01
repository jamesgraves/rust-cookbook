#[cfg(test)]
mod tests {
    use cookbook_utils::run_example;

    #[test]
    fn log_custom() {
        run_example("log-custom");
    }

    #[test]
    fn log_env_variable() {
        run_example("log-env-variable");
    }

    #[test]
    fn log_mod() {
        run_example("log-mod");
    }

    #[test]
    fn log_timestamp() {
        run_example("log-timestamp");
    }

}
