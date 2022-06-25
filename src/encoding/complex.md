# Structured Data

Add these crates to your own project with [`cargo-edit`]:

```sh
cargo add serde --features derive
cargo add byteorder serde_json toml
```

{{#include complex/complex-json.md}}

{{#include complex/toml.md}}

{{#include complex/endian-byte.md}}

{{#include ../links.md}}

[`cargo-edit`]: https://crates.io/crates/cargo-edit
