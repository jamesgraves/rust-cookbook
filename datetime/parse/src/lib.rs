#[cfg(test)]
mod tests {
    use cookbook_utils::run_example;

    #[test]
    fn billion_seconds() {
        run_example("billion_seconds");
    }

    #[test]
    fn current_rfc_2822_3339() {
        run_example("current_rfc_2822_3339");
    }

    #[test]
    fn current_utc() {
        run_example("current_utc");
    }

    #[test]
    fn custom_formats() {
        run_example("custom_formats");
    }
}
