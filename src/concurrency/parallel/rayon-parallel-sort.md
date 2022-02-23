## Sort a vector in parallel

[![rayon-badge]][rayon] [![rand-badge]][rand] [![cat-concurrency-badge]][cat-concurrency]

This example will sort in parallel a vector of Strings.

Allocate a vector of empty Strings. `par_iter_mut().for_each()` populates random
values in parallel.  Although [multiple options]
exist to sort an enumerable data type, [`par_sort_unstable`]
is usually faster than [stable sorting] algorithms.

```rust
{{#include examples/rayon-parallel-sort.rs}}
```

To truly exercise multiple threads of execution on multiple processor cores,
change `word_count` to be 100,000 or larger. Compare the execution time and CPU load
versus the single threaded versions of the same vector operations.

If the Rust Cookbook has been cloned, run:

```
cargo run --example rayon-parallel-sort
```

[`par_sort_unstable`]: https://docs.rs/rayon/*/rayon/slice/trait.ParallelSliceMut.html#method.par_sort_unstable
[multiple options]: https://docs.rs/rayon/*/rayon/slice/trait.ParallelSliceMut.html
[stable sorting]: https://docs.rs/rayon/*/rayon/slice/trait.ParallelSliceMut.html#method.par_sort
