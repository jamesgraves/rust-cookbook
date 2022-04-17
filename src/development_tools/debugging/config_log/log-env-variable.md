## Use a custom environment variable to set up logging

[![log-badge]][log] [![env_logger-badge]][env_logger] [![cat-debugging-badge]][cat-debugging]

[`Builder`] configures logging.

[`Builder::parse`] parses `MY_APP_LOG`
environment variable contents in the form of [`RUST_LOG`] syntax.
Then, [`Builder::init`] initializes the logger.
All these steps are normally done internally by [`env_logger::init`].

```rust
{{#include examples/log-env-variable.rs}}
```

Without setting `MY_APP_LOG`, the output of the example will be something like:

```sh
$ cargo run --quiet --example log-env-variable
[2022-04-17T00:52:42Z ERROR log_env_variable] this is an error message
```

When setting the environment variable `MY_APP_LOG` with a different log level, more
will be printed:

```sh
$ MY_APP_LOG=warn cargo run --quiet --example log-env-variable
[2022-04-17T00:56:51Z WARN  log_env_variable] warning message
[2022-04-17T00:56:51Z ERROR log_env_variable] this is an error message
```

[`env_logger::init`]: https://docs.rs/env_logger/*/env_logger/fn.init.html
[`Builder`]: https://docs.rs/env_logger/*/env_logger/struct.Builder.html
[`Builder::init`]: https://docs.rs/env_logger/*/env_logger/struct.Builder.html#method.init
[`Builder::parse_env`]: https://docs.rs/env_logger/*/env_logger/struct.Builder.html#method.parse_env
[`RUST_LOG`]: https://docs.rs/env_logger/*/env_logger/#enabling-logging
