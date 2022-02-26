#[cfg(test)]
mod tests {
    use cookbook_utils::*;

    #[test]
    fn sqlite3_initialization() {
        run_example("initialization");
        touch_file("initialization_complete");
    }
    #[test]
    fn sqlite3_insert_select() {
        look_for_file("initialization_complete", 20);
        run_example("insert_select");
        touch_file("insert_select_complete");
    }

    #[test]
    fn sqlite3_transactions() {
        look_for_file("insert_select_complete", 40);
        run_example("transactions");
    }
}
