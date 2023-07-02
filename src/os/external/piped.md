## Run piped external commands

[![std-badge]][std] [![cat-os-badge]][cat-os]

Shows up to the 10<sup>th</sup> biggest files and subdirectories in
the current working directory. It is equivalent to running:
`du -ah . | sort -hr | head -n 10`.

[`Command`]s represent a process. Output of a child process is captured with a
[`Stdio::piped`] between parent and child.

```rust,no_run
{{#include examples/pipelined_commands.rs}}
```

To run the example from the cookbook source code:

```shell,no_run
cargo run --example pipelined_commands
```

[`Command`]: https://doc.rust-lang.org/std/process/struct.Command.html
[`Stdio::piped`]: https://doc.rust-lang.org/std/process/struct.Stdio.html
