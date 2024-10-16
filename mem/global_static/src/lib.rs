#[cfg(test)]
mod tests {
    use cookbook_utils::run_example;

    #[test]
    fn global_static_lazy_constant() {
        run_example("lazy-constant");
    }
}
