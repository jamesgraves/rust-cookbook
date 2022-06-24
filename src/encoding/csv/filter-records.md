## Filter CSV records matching a predicate

[![csv-badge]][csv] [![cat-encoding-badge]][cat-encoding]

Returns _only_ the rows from `data` with a field that matches `query`. In this case, cities in the state of California (CA).

```rust
{{#include examples/filter-records.rs}}
```

_Disclaimer: this example has been adapted from [the csv crate tutorial](https://docs.rs/csv/*/csv/tutorial/index.html#filter-by-search)_.
