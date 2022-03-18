#[cfg(test)]
mod tests {
    use cookbook_utils::run_example;

    #[test]
    fn processor_cpu_count() {
        run_example("cpu-count");
    }
}
