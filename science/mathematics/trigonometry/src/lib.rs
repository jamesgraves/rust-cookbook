#[cfg(test)]
mod tests {
    use cookbook_utils::run_example;

    #[test]
    fn tan_sin_cos() {
        run_example("tan_sin_cos");
    }

    #[test]
    fn latitude_longitude() {
        run_example("latitude_longitude");
    }

    #[test]
    fn side_length() {
        run_example("side_length");
    }

}
