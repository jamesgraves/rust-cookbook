#[cfg(test)]
mod tests {
    use cookbook_utils::run_example;

    #[test]
    fn base64() {
        run_example("base64");
    }

    #[test]
    fn hex_encode() {
        run_example("hex-encode");
    }

    #[test]
    fn percent_encode() {
        run_example("percent-encode");
    }

    #[test]
    fn url_encode() {
        run_example("url-encode");
    }
}
