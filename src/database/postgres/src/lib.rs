#[cfg(test)]
mod tests {
    // You may enable these tests if you have the needed PostgreSQL
    // packages installed on your operating system and have the
    // PostgreSQL server running. Since they cannot be installed / run
    // on the continuous integration (CI) servers, all tests are
    // disabled by default.

    use cookbook_utils::*;

    // `touch_file()` and `look_for_file()` are needed to sequence the
    // unit tests.
    #[test]
    fn postgres_initialization() {
        run_example("pg_initialization");
        touch_file("initialization_complete");
    }

    #[test]
    fn postgres_insert_select() {
        look_for_file("initialization_complete", 20);
        run_example("pg_insert_select");
        touch_file("insert_select_complete");
    }

    #[test]
    fn postgres_transactions() {
        look_for_file("insert_select_complete", 40);
        run_example("pg_transactions");
    }
}
