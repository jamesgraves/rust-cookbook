## Redirect both stdout and stderr of child process to the same file

[![std-badge]][std] [![cat-os-badge]][cat-os]

Spawns a child process and redirects `stdout` and `stderr` to the same
file. It follows the same idea as [run piped external
commands](#run-piped-external-commands), however [`process::Stdio`]
writes to a specified file.  [`File::try_clone`] references the same file handle
for `stdout` and `stderr`. It will ensure that both handles write with the same
cursor position.

The below recipe is equivalent to run the Unix shell command
`ls /tmp /tmp/oops > /tmp/out.txt 2>&1`.

Since the `/tmp/oops` file doesn't exist (unless you create it!), the `ls` command
will generate an error on `stderr` as well as the normal output on `stdout`.

```rust,no_run
{{#include examples/command_stderr.rs}}
```

To run this from the cookbook source code directory:

```shell,no_run
cargo run --example command_stderr
```

Examine the generated output:

```shell,no_run
cat /tmp/out.txt
```

[`File::try_clone`]: https://doc.rust-lang.org/std/fs/struct.File.html#method.try_clone
[`process::Stdio`]: https://doc.rust-lang.org/std/process/struct.Stdio.html
