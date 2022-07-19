#[cfg(test)]
mod tests {
    use cookbook_utils::run_example;
    // endian-byte.rs  json.rs  serde-toml.rs  toml.rs

    #[test]
    fn endian_byte() {
        run_example("endian-byte");
    }

    #[test]
    fn complex_json() {
        run_example("complex-json");
    }

    #[test]
    fn serde_toml() {
        run_example("serde-toml");
    }

    #[test]
    fn encoding_toml() {
        run_example("toml");
    }
}
