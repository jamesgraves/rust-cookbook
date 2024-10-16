#[cfg(test)]
mod tests {
    use cookbook_utils::run_example;

    #[test]
    fn from_str() {
        run_example("from_str");
    }

    #[test]
    fn unicode_graphemes() {
        run_example("unicode_graphemes");
    }
}
