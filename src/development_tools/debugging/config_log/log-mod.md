## Enable log levels per module

[![log-badge]][log] [![env_logger-badge]][env_logger] [![cat-debugging-badge]][cat-debugging]

Creates two modules `foo` and nested `foo::bar` with logging directives
controlled separately with [`RUST_LOG`] environmental variable.

```rust
{{#include examples/log-mod.rs}}
```

[`RUST_LOG`] environment variable controls [`env_logger`][env_logger] output.
Module declarations take comma separated entries formatted like
`path::to::module=log_level`. From the cloned Rust Cookbook repository,
run the example as follows:

```bash
RUST_LOG="warn,log_mod::foo=info,log_mod::foo::bar=debug" cargo run --example log-mod
```

This sets the default [`log::Level`] to `warn` for the entire
application, module `foo` in the application to `info`, and sub-module
`foo::bar` to `debug`.

Note that because the example filename has a minus sign ('-') in it,
which is converted to and underscore ('_') because module names cannot
have a minus sign.

```bash
[2022-05-21T12:30:34Z WARN  log_mod] [root] warn
[2022-05-21T12:30:34Z WARN  log_mod::foo] [foo] warn
[2022-05-21T12:30:34Z INFO  log_mod::foo] [foo] info
[2022-05-21T12:30:34Z WARN  log_mod::foo::bar] [bar] warn
[2022-05-21T12:30:34Z INFO  log_mod::foo::bar] [bar] info
[2022-05-21T12:30:34Z DEBUG log_mod::foo::bar] [bar] debug
```

[`log::Level`]: https://docs.rs/log/*/log/enum.Level.html
[`RUST_LOG`]: https://docs.rs/env_logger/*/env_logger/#enabling-logging
