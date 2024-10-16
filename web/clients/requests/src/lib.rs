#[cfg(test)]
mod tests {
    use cookbook_utils::run_example;

    #[test]
    fn web_client_request_get() {
        run_example("web_client_request_get");
    }
    #[test]
    fn web_client_request_headers() {
        run_example("web_client_request_headers");
    }
}
