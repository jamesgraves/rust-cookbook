## Verifying tan is equal to sin divided by cos

[![std-badge]][std] [![cat-science-badge]][cat-science]

Add the [`approx`] crate to your own project:

```shell
cargo add approx
```

Verifies tan(x) is equal to sin(x)/cos(x) for x = 6.

While, in this particular case, the results of these computations
produce an identical result, in general you should not be using
the standard equality operator to compare to floating point numbers.
Instead, this example uses the [`approx`] crate for floating point
comparisons.

You can experiment with this by enabling the assertion, and changing
the `x` value to something else like 0.1 for example.

```rust
{{#include examples/tan_sin_cos.rs}}
```

[`approx`]: https://docs.rs/approx/*/approx/index.html
