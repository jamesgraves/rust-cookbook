# CSV processing

Add these crates to your own project using [`cargo-edit`]:

```sh
cargo add csv serde anyhow
```

{{#include csv/read.md}}

{{#include csv/delimiter.md}}

{{#include csv/filter-records.md}}

{{#include csv/invalid.md}}

{{#include csv/serialize.md}}

{{#include csv/serde-serialize.md}}

{{#include csv/transform.md}}

{{#include ../links.md}}

[`cargo-edit`]: https://crates.io/crates/cargo-edit
