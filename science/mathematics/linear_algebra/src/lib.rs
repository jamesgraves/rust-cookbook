#[cfg(test)]
mod tests {
    use cookbook_utils::run_example;

    #[test]
    fn add_matrices() {
        run_example("add_matrices");
    }

    #[test]
    fn deserialize_matrix() {
        run_example("deserialize_matrix");
    }

    #[test]
    fn invert_matrix() {
        run_example("invert_matrix");
    }

    #[test]
    fn multiply_matrices() {
        run_example("multiply_matrices");
    }

    #[test]
    fn multiply_scalar_vector_matrix() {
        run_example("multiply_scalar_vector_matrix");
    }

    #[test]
    fn vector_norm() {
        run_example("vector_norm");
    }

}
