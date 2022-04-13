#[cfg(test)]
mod tests {
    use cookbook_utils::*;

    #[test]
    fn read_write_memmap() {
        run_example("memmap");
    }

    #[test]
    fn read_write_buffered() {
        run_example("buffered");
        run_example("same-file");
    }
}
