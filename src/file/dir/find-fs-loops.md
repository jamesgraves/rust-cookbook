## Find loops for a given path

[![same_file-badge]][same_file] [![cat-filesystem-badge]][cat-filesystem]

Because it uses symbolic links, this example will run on:

* Linux / Unix
* MacOS
* Windows Services for Linux (WSL)

Add the `same-file` crate to your own project:

```
cargo add same-file
```

Use [`same_file::is_same_file`] to detect loops for a given path.
For example, a loop could be created on a Unix system via symlinks:

```bash
mkdir -p /tmp/foo/bar/baz
ln -s /tmp/foo/  /tmp/foo/bar/baz/qux
```

The following would assert that a loop exists.

```rust,no_run
{{#include examples/find-fs-loops.rs}}
```

The Unix / Linux `find` utility also detects the loop:

```
$ find /tmp/foo | xargs file
/tmp/foo:             directory
/tmp/foo/bar:         directory
/tmp/foo/bar/baz:     directory
/tmp/foo/bar/baz/qux: symbolic link to /tmp/foo
$ find -L /tmp/foo
find: File system loop detected; ‘/tmp/foo/bar/baz/qux’ is part of the same file system loop as ‘/tmp/foo’.
/tmp/foo
/tmp/foo/bar
/tmp/foo/bar/baz
```

[`same_file::is_same_file`]: https://docs.rs/same-file/*/same_file/fn.is_same_file.html
