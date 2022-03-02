#[cfg(test)]
mod tests {
    // You may enable these tests if you have the needed sqlite3 packages installed on your
    // operating system. Since they cannot be installed / run on the continuous integration (CI)
    // servers, all tests are disabled by default.

    use cookbook_utils::*;

    #[test]
    fn postgres_initialization() {
        run_example("initialization");
        touch_file("initialization_complete");
    }

    #[test]
    fn postgres_insert_select() {
        look_for_file("initialization_complete", 20);
        run_example("insert_select");
        touch_file("insert_select_complete");
    }

    #[test]
    fn postgres_transactions() {
        look_for_file("insert_select_complete", 40);
        run_example("transactions");
    }
}
