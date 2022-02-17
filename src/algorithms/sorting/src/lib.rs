

#[cfg(test)]
mod tests {
    use cookbook_utils::run_example;

    #[test]
    fn sort_basic() {
        run_example("sort_basic");
    }

    #[test]
    fn sort_float() {
        run_example("sort_float");
    }

    #[test]
    fn sort_struct() {
        run_example("sort_struct");
    }
}


