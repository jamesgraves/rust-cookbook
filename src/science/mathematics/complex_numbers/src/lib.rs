#[cfg(test)]
mod tests {
    use cookbook_utils::run_example;

    /*
     * add_complex.rs  create_complex.rs  mathematical_functions.rs
    */

    #[test]
    fn add_complex() {
        run_example("add_complex");
    }

    #[test]
    fn create_complex() {
        run_example("create_complex");
    }

    #[test]
    fn mathematical_functions() {
        run_example("mathematical_functions");
    }

}
