#[cfg(test)]
mod tests {
    use cookbook_utils::run_example;

    #[test]
    fn crossbeam_complex() {
        run_example("crossbeam-complex");
    }

    #[test]
    fn crossbeam_spawn() {
        run_example("crossbeam-spawn");
    }

    #[test]
    fn crossbeam_spsc() {
        run_example("crossbeam-spsc");
    }

    #[test]
    fn global_mut_state() {
        run_example("global-mut-state");
    }

    #[test]
    fn threadpool_fractal() {
        run_example("threadpool-fractal");
    }

    #[test]
    fn threadpool_walk() {
        run_example("threadpool-walk");
    }
}
