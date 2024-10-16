#[cfg(test)]
mod tests {
    use cookbook_utils::*;

    #[test]
    fn dir_calc_file_sizes() {
        run_example("calc-file-sizes");
    }

    #[test]
    fn dir_duplicate_name() {
        run_example("duplicate-name");
    }

    #[test]
    fn dir_file_modification_time() {
        run_example("file-modification-time");
    }

    #[test]
    fn dir_find_all_png_files() {
        run_example("find-all-png-files");
    }

    #[test]
    fn dir_find_file() {
        run_example("find-file");
    }

    // FIXME: make option only for Unix
    #[test]
    fn dir_find_fs_loops() {
        run_example("find-fs-loops");
    }

    #[test]
    fn dir_ignore_case() {
        run_example("ignore-case");
    }

    #[test]
    fn dir_skip_dot_files() {
        run_example("skip-dot-files");
    }

}
