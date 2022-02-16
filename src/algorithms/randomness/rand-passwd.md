## Create random passwords from a set of alphanumeric characters

[![rand-badge]][rand] [![cat-os-badge]][cat-os]

Randomly generates a string of given length ASCII characters in the range `A-Z,
a-z, 0-9`, with [`Alphanumeric`] sample.

```rust
{{#include ../../../algorithms/randomness/examples/random_string.rs}}
```

[`Alphanumeric`]: https://docs.rs/rand/*/rand/distributions/struct.Alphanumeric.html
