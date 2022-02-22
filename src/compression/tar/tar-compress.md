## Compress a directory into tarball

[![flate2-badge]][flate2] [![tar-badge]][tar] [![cat-compression-badge]][cat-compression]

Create some test files, and then compress those into a tar archive file named `archive.tar.gz`.

In the example, first some test files are created in two separate directories.

For the actual tar archive, the first step is to create a new [`File`],
which is then wrapped in [`GzEncoder`] and then in [`tar::Builder`]. After that,
the test data directories (and recursively the files within them) are added
to the tar archive file via the [`Builder::append_dir_all`] function.
The [`GzEncoder`] is responsible for transparently compressing the
data prior to writing it into `archive.tar.gz`.

```rust,no_run
{{#include examples/tar-compress.rs}}
```

As with other examples, the error handling is very simplified and uses
`Result<(), Box<dyn Error>>` as the return type for both the function
`echo()` as well as `main()` itself. See the [`Error`] page for better
approaches to error handling in applications and libraries.

Since this creates files on the filesystem, it cannot be run on the Rust
Playground. You can either clone the Rust Cookbook project, or just
create a new Cargo project and replace the `main.rs` with the contents above.

Run this example from the Rust Cookbook project:

```
cargo run --example tar-compress
```

To examine the result, the `tar` command line utility can be used. It is
installed by default on most Linux systems and MacOS. `tar` is also
available inside WSL on Windows.

Examine the newly created archive via:

```
tar xvf archive.tar.gz
```

The output will be similar to this:

```
drwxrwsr-x 1000/1000         0 2022-02-20 11:22 temp1-renamed/
-rw-rw-r-- 1000/1000         7 2022-02-20 11:22 temp1-renamed/two.txt
-rw-rw-r-- 1000/1000         6 2022-02-20 11:22 temp1-renamed/one.txt
drwxrwsr-x 1000/1000         0 2022-02-20 11:22 temp7/
-rw-rw-r-- 1000/1000         7 2022-02-20 11:22 temp7/four.txt
-rw-rw-r-- 1000/1000         6 2022-02-20 11:22 temp7/three.txt
```

[`Builder::append_dir_all`]: https://docs.rs/tar/*/tar/struct.Builder.html#method.append_dir_all
[`File`]: https://doc.rust-lang.org/std/fs/struct.File.html
[`GzEncoder`]: https://docs.rs/flate2/*/flate2/write/struct.GzEncoder.html
[`tar::Builder`]: https://docs.rs/tar/*/tar/struct.Builder.html
