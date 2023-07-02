#[cfg(test)]
mod tests {
    use cookbook_utils::run_example;

    #[test]
    fn command_read_stdout() {
        run_example("command_read_stdout");
    }

    #[test]
    fn command_input_output() {
        run_example("command_input_output");
    }

    #[test]
    fn command_stderr() {
        run_example("command_stderr");
    }

    #[test]
    fn pipelined_commands() {
        run_example("pipelined_commands");
    }

    #[test]
    fn process_output() {
        run_example("process_output");
    }

    #[test]
    fn read_env_variable() {
        run_example("read_env_variable");
    }

}
