#[cfg(test)]
mod tests {
    use cookbook_utils::run_example;

    #[test]
    fn basic_usage() {
        run_example("basic_usage");
    }

    #[test]
    fn gen_range() {
        run_example("gen_range");
    }

    #[test]
    fn uniform_distribution() {
        run_example("uniform_distribution");
    }

    #[test]
    fn generate_password() {
        run_example("generate_password");
    }

    #[test]
    fn normal_distribution() {
        run_example("normal_distribution");
    }

    #[test]
    fn random_string() {
        run_example("random_string");
    }

    #[test]
    fn tuple_distribution() {
        run_example("tuple_distribution");
    }
}
