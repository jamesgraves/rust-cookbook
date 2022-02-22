#[cfg(test)]
mod tests {
    use cookbook_utils::run_example;
    #[test]
    fn tar_compress() {
        run_example("tar-compress");
    }

    #[test]
    fn tar_decompress() {
        run_example("tar-decompress");
    }
}
