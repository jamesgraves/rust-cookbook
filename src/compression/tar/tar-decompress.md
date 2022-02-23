## Decompress a tarball

[![flate2-badge]][flate2] [![tar-badge]][tar] [![cat-compression-badge]][cat-compression]

Decompress (using [`GzDecoder`]) and
extract ([`Archive::unpack`]) all files from a compressed tar archive file
named `archive.tar.gz` located in the current working directory
to the same location.

```rust,no_run
{{#include examples/tar-decompress.rs}}
```

Since these examples interact with the filesystem, they can't be run on the
Rust Playground. Either clone the Rust Cookbook repository or copy and paste
the example code into a newly created project.

Run the tar compression example above to create a suitable `archive.tar.gz`
file. Then remove the temporary files and directories if the
`tar-compress.rs` with:

```
rm -r temp1 temp7
```

If the Rust Cookbook repository has been cloned, run:

```
cargo run --example tar-compress
```

If using the previously generated example, the `temp1-renamed` and `temp7` directories
and files have been created.

[`Archive::unpack`]: https://docs.rs/tar/*/tar/struct.Archive.html#method.unpack
[`GzDecoder`]: https://docs.rs/flate2/*/flate2/read/struct.GzDecoder.html
