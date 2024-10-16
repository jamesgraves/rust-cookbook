#[cfg(test)]
mod tests {
    use cookbook_utils::run_example;

    #[test]
    fn clap_basic() {
        run_example("clap_basic");
    }

    #[test]
    fn clap_with_pathbuf() {
        run_example("clap_with_pathbuf");
    }
}
