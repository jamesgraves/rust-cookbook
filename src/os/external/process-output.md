## Run an external command and process stdout

[![regex-badge]][regex] [![cat-os-badge]][cat-os] [![cat-text-processing-badge]][cat-text-processing]

Add the `regex` crate to your own project:

```shell,no_run
cargo add regex
```

Runs `git log --oneline` as an external [`Command`] and inspects its [`Output`]
using [`Regex`] to get the hash and message of the last 5 commits.

```rust,no_run
{{#include examples/process_output.rs}}
```

Run the example inside a git repository:

```shell,no_run
cargo run --example process_output
```

[`Command`]: https://doc.rust-lang.org/std/process/struct.Command.html
[`Output`]: https://doc.rust-lang.org/std/process/struct.Output.html
[`Regex`]: https://docs.rs/regex/*/regex/struct.Regex.html
