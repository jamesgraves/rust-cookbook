#[cfg(test)]
mod tests {
    // You may enable these tests if you have the needed PostgreSQL
    // packages installed on your operating system and have the
    // PostgreSQL server running. Since they cannot be installed / run
    // on the continuous integration (CI) servers, all tests are
    // disabled by default.

    use cookbook_utils::*;

    #[test]
    fn postgres_sequence() {
        run_example("pg_initialization");
        run_example("pg_insert_select");
        run_example("pg_transactions");
    }
}
