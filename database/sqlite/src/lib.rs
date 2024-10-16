#[cfg(test)]
mod tests {
    use cookbook_utils::*;

    #[test]
    fn sqlite3_sequence() {
        run_example("initialization");
        run_example("insert_select");
        run_example("transactions");
    }
}
