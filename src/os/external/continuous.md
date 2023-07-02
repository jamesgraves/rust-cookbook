## Continuously process child process' outputs

[![std-badge]][std] [![cat-os-badge]][cat-os]

In [Run an external command and process stdout](#run-an-external-command-and-process-stdout),
processing doesn't start until external [`Command`] is finished.
The recipe below calls [`Stdio::piped`] to create a pipe, and reads
`stdout` continuously as soon as the [`BufReader`] is updated.

The below recipe is equivalent to the Unix shell command
`lspci | grep bridge`.

```rust,no_run
{{#include examples/command_read_stdout.rs}}
```

Run this example from the cookbook source code directory:

```shell,no_run
cargo run --example command_read_stdout
```

[`BufReader`]: https://doc.rust-lang.org/std/io/struct.BufReader.html
[`Command`]: https://doc.rust-lang.org/std/process/struct.Command.html
[`Stdio::piped`]: https://doc.rust-lang.org/std/process/struct.Stdio.html
