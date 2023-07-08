## Replace all occurrences of one text pattern with another pattern.

[![regex-badge]][regex] [![lazy_static-badge]][lazy_static] [![cat-text-processing-badge]][cat-text-processing]

Replaces all occurrences of the American English date pattern
*MM/DD/YYYY* with the equivalent ISO 8601 standard *YYYY-MM-DD* date format.
For example `01/15/2013` becomes `2013-01-15`.

The method [`Regex::replace_all`] replaces all occurrences of the whole regex.
`&str` implements the `Replacer` trait which allows variables like `$abcde` to
refer to corresponding named capture groups `(?P<abcde>REGEX)` from the search
regex. See the [replacement string syntax] for examples and escaping detail.

```rust
{{#include examples/replace_date_format.rs}}
```

[`Regex::replace_all`]: https://docs.rs/regex/*/regex/struct.Regex.html#method.replace_all

[replacement string syntax]: https://docs.rs/regex/*/regex/struct.Regex.html#replacement-string-syntax
