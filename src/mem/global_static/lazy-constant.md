## Declare lazily evaluated constant

[![lazy_static-badge]][lazy_static] [![cat-caching-badge]][cat-caching] [![cat-rust-patterns-badge]][cat-rust-patterns]

Add the `lazy_static` crate to your own project:

```
cargo add lazy_static
```

Declares a lazily evaluated constant [`HashMap`]. The [`HashMap`] will
be evaluated once and stored behind a global static reference.

```rust
{{#include examples/lazy-constant.rs}}
```

[`HashMap`]: https://doc.rust-lang.org/std/collections/struct.HashMap.html
