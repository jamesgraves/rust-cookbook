#[cfg(test)]
mod tests {
    use cookbook_utils::run_example;

    #[test]
    fn sha_digest() {
        run_example("sha-digest");
    }

    #[test]
    fn hmac() {
        run_example("hmac");
    }
}
