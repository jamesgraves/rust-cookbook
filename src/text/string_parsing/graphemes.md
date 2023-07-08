## Collect Unicode Graphemes

[![unicode-segmentation-badge]][`unicode-segmentation`] [![cat-text-processing-badge]][cat-text-processing]

Add this crate to your own project:

```
cargo add unicode-segmentation
```

Collect individual Unicode graphemes from UTF-8 string using the 
[`UnicodeSegmentation::graphemes`] function from the [`unicode-segmentation`] crate.

```rust
{{#include examples/unicode_graphemes.rs}}
```

[`UnicodeSegmentation::graphemes`]: https://docs.rs/unicode-segmentation/*/unicode_segmentation/trait.UnicodeSegmentation.html#tymethod.graphemes
[`unicode-segmentation`]: https://docs.rs/unicode-segmentation/latest/unicode_segmentation/
