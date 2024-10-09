## Serialize and deserialize unstructured JSON

[![serde-json-badge]][serde-json] [![cat-encoding-badge]][cat-encoding]

The [`serde_json`] crate provides a [`from_str`] function to parse a `&str` of
JSON.

Unstructured JSON can be parsed into a universal [`serde_json::Value`] type that
is able to represent any valid JSON data.

The example below shows a `&str` (reference to a string slice) of JSON being parsed.  The expected value is declared using the [`json!`] macro.

```rust
{{#include examples/complex-json.rs}}
```

Note the use of [`raw string`]s with the `r#"` and `"#`.

[`from_str`]: https://docs.serde.rs/serde_json/fn.from_str.html
[`json!`]: https://docs.serde.rs/serde_json/macro.json.html
[`serde_json`]: https://docs.serde.rs/serde_json/
[`serde_json::Value`]: https://docs.serde.rs/serde_json/enum.Value.html
[`raw string`]: https://doc.rust-lang.org/rust-by-example/std/str.html
