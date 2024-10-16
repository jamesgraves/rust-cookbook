## Sort a Vector of Integers

[![std-badge]][std] [![cat-science-badge]][cat-science]

This example sorts a Vector of integers via [`vec::sort`]. Alternative would
be to use [`vec::sort_unstable`] which can be faster, but does not preserve
the order of equal elements.

```rust
{{#include examples/sort-basic.rs}}
```

[`vec::sort`]: https://doc.rust-lang.org/std/vec/struct.Vec.html#method.sort
[`vec::sort_unstable`]: https://doc.rust-lang.org/std/vec/struct.Vec.html#method.sort_unstable
