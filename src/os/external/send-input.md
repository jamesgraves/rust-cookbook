## Run an external command passing it stdin and check for an error code

[![std-badge]][std] [![cat-os-badge]][cat-os]

This example requires a `python3` interpreter installed.

Opens the `python` interpreter using an external [`Command`] and passes it a
python statement for execution. [`Output`] of statement is then parsed.

```rust,no_run
{{#include examples/command_input_output.rs}}
```

Run this example from the cookbook source code directory:

```shell,no_run
cargo run --example command_input_output
```

[`Command`]: https://doc.rust-lang.org/std/process/struct.Command.html
[`Output`]: https://doc.rust-lang.org/std/process/struct.Output.html
