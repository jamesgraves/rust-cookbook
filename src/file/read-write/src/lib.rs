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
        touch_file("buffered_complete");
    }

    #[test]
    fn read_write_same_file() {
        look_for_file("buffered_complete", 20);
        run_example("same-file");
    }
}
