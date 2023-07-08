#[cfg(test)]
mod tests {
    use cookbook_utils::run_example;

    #[test]
    fn email_address_format() {
        run_example("email_address_format");
    }

    #[test]
    fn extract_hashtags() {
        run_example("extract_hashtags");
    }

    #[test]
    fn extract_phone_numbers() {
        run_example("extract_phone_numbers");
    }

    #[test]
    fn filter_log() {
        run_example("filter_log");
    }

    #[test]
    fn replace_date_format() {
        run_example("replace_date_format");
    }
}
