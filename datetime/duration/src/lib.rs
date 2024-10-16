#[cfg(test)]
mod tests {
    use cookbook_utils::run_example;

    #[test]
    fn duration_checked() {
        run_example("checked");
    }

    #[test]
    fn duration_profile() {
        run_example("profile");
    }

    #[test]
    fn duration_timezone() {
        run_example("timezone");
    }
}
