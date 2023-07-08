## Filter a log file by matching multiple regular expressions

[![regex-badge]][regex] [![cat-text-processing-badge]][cat-text-processing]


Reads file contents (stored as a string in the program itself)  and only
outputs the lines containing “version X.X.X”, some IP address followed by port 443
(e.g. “192.168.0.1:443”), or a specific warning.

The example can be modified to read from an actual file.

A [`regex::RegexSetBuilder`] composes a [`regex::RegexSet`].
Since backslashes are very common in regular expressions, using
[raw string literals] makes them more readable.

```rust
{{#include examples/filter_log.rs}}
```

[`regex::RegexSet`]: https://docs.rs/regex/*/regex/struct.RegexSet.html
[`regex::RegexSetBuilder`]: https://docs.rs/regex/*/regex/struct.RegexSetBuilder.html

[raw string literals]: https://doc.rust-lang.org/reference/tokens.html#raw-string-literals
