## Serialize records to CSV using Serde

[![csv-badge]][csv] [![serde-badge]][serde] [![cat-encoding-badge]][cat-encoding]

The following example shows how to serialize custom structs as CSV records using
the [serde] crate.  Here the csv is being directly to STDOUT because the
writer is created from `io::stdout()`, but this could instead be
anything that supports the [`std::io::Write`] trait. As with any file-like object,
it is good practice to call `flush()` to ensure all buffered data has made it to the
final destination (stable storage in case of a file, for example).

```rust
{{#include examples/serde-serialize.rs}}
```

[`std::io::Write`]: https://doc.rust-lang.org/stable/std/io/trait.Write.html
