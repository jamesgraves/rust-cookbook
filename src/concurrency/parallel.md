# Parallel Tasks

Add the `rayon` crate to your own project:

```
cargo add rayon
```
Other crates needed for the example code:

```
cargo add glob rand image
```

{{#include parallel/rayon-iter-mut.md}}

{{#include parallel/rayon-any-all.md}}

{{#include parallel/rayon-parallel-search.md}}

{{#include parallel/rayon-parallel-sort.md}}

{{#include parallel/rayon-map-reduce.md}}

{{#include parallel/rayon-thumbnails.md}}

{{#include ../links.md}}
