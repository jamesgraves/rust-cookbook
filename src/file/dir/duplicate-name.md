## Recursively find duplicate file names

[![walkdir-badge]][walkdir] [![cat-filesystem-badge]][cat-filesystem]

Find recursively in the current directory any duplicate filenames,
printing them only once.

A [`HashMap`] is used to collect the filenames, the file name itself is
the key, and the count of how many times that filename has been seen is
the value stored in the HashMap. Any errors encountered, such as a
directory that does not have read + execute permissions for the current
user are silently ignored.

```rust,no_run
{{#include examples/duplicate-name.rs}}
```
