#[cfg(test)]
mod tests {
    use cookbook_utils::run_example;
    #[test]
    fn term_basic() {
        run_example("term_basic");
    }

    #[test]
    fn term_color() {
        run_example("term_color");
    }

    #[test]
    fn term_style() {
        run_example("term_style");
    }
}
