# Linear Algebra

Add these crates to your own project with the `cargo add` command:

```shell
cargo add approx
cargo add ndarray -F approx
cargo add nalgebra -F serde-serialize
```

{{#include linear_algebra/add-matrices.md}}
{{#include linear_algebra/multiply-matrices.md}}
{{#include linear_algebra/multiply-scalar-vector-matrix.md}}
{{#include linear_algebra/vector-comparison.md}}
{{#include linear_algebra/vector-norm.md}}
{{#include linear_algebra/invert-matrix.md}}
{{#include linear_algebra/deserialize-matrix.md}}

{{#include ../../links.md}}
