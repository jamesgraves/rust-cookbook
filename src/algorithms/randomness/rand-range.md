## Generate random numbers within a range

[![rand-badge]][rand] [![cat-science-badge]][cat-science]

Generates a random value within half-open `[0, 10)` range (not including `10`) with [`Rng::gen_range`].

```rust,edition2021
{{#include ../../../algorithms/randomness/examples/gen_range.rs}}
```

[`Uniform`] can obtain values with [uniform distribution].
This has the same effect, but may be faster when repeatedly generating numbers
in the same range.

```rust,edition2021
{{#include ../../../algorithms/randomness/examples/uniform_distribution.rs}}
```

[`Uniform`]: https://docs.rs/rand/*/rand/distributions/uniform/struct.Uniform.html
[`Rng::gen_range`]: https://doc.rust-lang.org/rand/*/rand/trait.Rng.html#method.gen_range
[uniform distribution]: https://en.wikipedia.org/wiki/Uniform_distribution_(continuous)
