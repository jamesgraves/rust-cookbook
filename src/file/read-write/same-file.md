## Avoid writing and reading from a same file

[![same_file-badge]][same_file] [![cat-filesystem-badge]][cat-filesystem]

Add this crate to your own project:

```
cargo add same_file
```

Use [`same_file::Handle`] to a file that can be tested for equality with
other handles. In this example, the handles of file to be read from and
to be written to are tested for equality.

```rust
{{#include examples/same-file.rs}}
```

```bash
cargo run --example same-file
```
displays the contents of the file content.txt.

```bash
cargo run --example same-file >> ./content.txt
```
errors because the two files are same.

[`same_file::Handle`]: https://docs.rs/same-file/*/same_file/struct.Handle.html
