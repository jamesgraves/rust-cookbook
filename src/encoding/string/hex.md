## Encode and decode hex

[![data-encoding-badge]][data-encoding] [![cat-encoding-badge]][cat-encoding]

The [`data_encoding`] crate provides a `HEXUPPER::encode` method which
takes a `&[u8]` and returns a `String` containing the hexadecimal
representation of the data.

Similarly, a `HEXUPPER::decode` method is provided which takes a `&[u8]` and
returns a `Vec<u8>` if the input data is successfully decoded.

The example below coverts `&[u8]` data to hexadecimal equivalent.  Compares this
value to the expected value.

```rust
{{#include examples/hex-encode.rs}}
```

Note that since the `original` is just raw bytes, the conversion to UTF8 may
fail, so the `Result` must be checked before printing. Since this variable
was initialized from a program constant, it is fine to use just use
`.unwrap()`, but in general any error from such a conversion should be
explicitly handled.

[`data_encoding`]: https://docs.rs/data-encoding/*/data_encoding/
