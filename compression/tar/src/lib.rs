#[cfg(test)]
mod tests {
    use cookbook_utils::run_example;
    #[test]
    fn tar_compress_decompress() {
        run_example("tar-compress");
        run_example("tar-decompress");
    }
}
