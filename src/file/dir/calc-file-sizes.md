## Recursively calculate file sizes at given depth

[![walkdir-badge]][walkdir] [![cat-filesystem-badge]][cat-filesystem]


Recursion depth can be flexibly set by [`WalkDir::min_depth`] & [`WalkDir::max_depth`] methods.
Calculates sum of all file sizes to 3 subfolders depth, ignoring files in the root folder.

```rust,norun
{{#include examples/calc-file-sizes.rs}}
```

`WalkDir` uses the builder pattern to set the arguments, then this struct is converted to an iterator.
For each file that exists and has metadata, the size is retrieved and then summed.

[`WalkDir::max_depth`]: https://docs.rs/walkdir/*/walkdir/struct.WalkDir.html#method.max_depth
[`WalkDir::min_depth`]: https://docs.rs/walkdir/*/walkdir/struct.WalkDir.html#method.min_depth
