#[cfg(test)]
mod tests {
    use cookbook_utils::run_example;

    #[test]
    fn standard_deviation() {
        run_example("standard_deviation");
    }

    #[test]
    fn stats_count_set() {
        run_example("stats_count_set");
    }

    #[test]
    fn stats_mean() {
        run_example("stats_mean");
    }

    #[test]
    fn stats_quickselect() {
        run_example("stats_quickselect");
    }

}
