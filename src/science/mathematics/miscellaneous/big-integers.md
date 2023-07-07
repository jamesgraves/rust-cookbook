## Big integers

[![num-badge]][num] [![cat-science-badge]][cat-science]

Add the `num_bigint` crate to your own project:

```
cargo add num_bigint
```

Calculation for integers exceeding 128 bits are possible with [`BigInt`].

```rust
{{#include examples/big_integers.rs}}
```

[`BigInt`]: https://docs.rs/num-bigint/latest/num_bigint/
