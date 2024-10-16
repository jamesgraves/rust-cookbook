#[cfg(test)]
mod tests {
    use cookbook_utils::run_example;

    #[test]
    fn log_custom_logger() {
        run_example("log-custom-logger");
    }

    #[test]
    fn log_debug() {
        run_example("log-debug");
    }

    #[test]
    fn log_error() {
        run_example("log-error");
    }

    #[test]
    fn log_stdout() {
        run_example("log-stdout");
    }

    #[test]
    fn log_syslog() {
        run_example("log-syslog");
    }

}
