#[cfg(test)]
mod tests {
    use cookbook_utils::run_example;

    #[test]
    fn delimiter() {
        run_example("delimiter");
    }

    #[test]
    fn invalid_field() {
        run_example("invalid-field");
    }

    #[test]
    fn read_weakly_typed() {
        run_example("read-weakly-typed");
    }

    #[test]
    fn serialize_tuple() {
        run_example("serialize-tuple");
    }

    #[test]
    fn filter_records() {
        run_example("filter-records");
    }

    #[test]
    fn read_strongly_typed() {
        run_example("read-strongly-typed");
    }

    #[test]
    fn serde_serialize() {
        run_example("serde-serialize");
    }

    #[test]
    fn transform_color() {
        run_example("transform-color");
    }

}
