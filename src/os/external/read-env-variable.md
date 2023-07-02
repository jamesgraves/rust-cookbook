## Read Environment Variable 

[![std-badge]][std] [![cat-os-badge]][cat-os]

Reads an environment variable via [std::env::var].

```rust,no_run
{{#include examples/read_env_variable.rs}}
```

Run this example from the cookbook source code directory:

```shell,no_run
cargo run --example read_env_variable
```

Try creating a `CONFIG` environment variable with the filename of 
a file that exists:

```shell,no_run
export CONFIG="/tmp/my_config"
echo "hello there" > /tmp/my_config
cargo run --example read_env_variable
```

[std::env::var]: https://doc.rust-lang.org/std/env/fn.var.html
