#[cfg(test)]
mod tests {
    use cookbook_utils::run_example;

    #[test]
    fn rayon_any_all() {
        run_example("rayon-any-all");
    }

    #[test]
    fn rayon_iter_mut() {
        run_example("rayon-iter-mut");
    }

    #[test]
    fn rayon_map_reduce() {
        run_example("rayon-map-reduce");
    }

    #[test]
    fn rayon_parallel_search() {
        run_example("rayon-parallel-search");
    }

    #[test]
    fn rayon_parallel_sort() {
        run_example("rayon-parallel-sort");
    }

    #[test]
    fn rayon_thumbnails() {
        run_example("rayon-thumbnails");
    }
}
