## Handle invalid CSV data with Serde

[![csv-badge]][csv] [![serde-badge]][serde] [![cat-encoding-badge]][cat-encoding]

CSV files often contain invalid data. In this example, not all the
values used for `id` in the data can be parsed correctly as unsigned integers.
This would normally generate a parse error.

The `csv` crate provides a custom deserializer, [`csv::invalid_option`],
which automatically converts invalid data to `None` values. Try
commenting out the `serde` macro invocation in the definition of the
struct `Record` before the `id: Option<u64>` line.

```rust
{{#include examples/invalid-field.rs}}
```

[`csv::invalid_option`]: https://docs.rs/csv/*/csv/fn.invalid_option.html
