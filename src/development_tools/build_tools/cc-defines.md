## Compile a C library while setting custom defines

[![cc-badge]][cc] [![cat-development-tools-badge]][cat-development-tools]

It is simple to build bundled C code with custom defines using [`cc::Build::define`].
The method takes an [`Option`] value, so it is possible to create defines such as `#define APP_NAME "a cookbook demo"`
as well as `#define WELCOME` (pass `None` as the value for a value-less define). This example builds
a bundled C file with dynamic defines set in `build.rs` and prints "**Welcome to a cookbook demo - version 0.1.0**"
when run. Cargo sets some [environment variables][cargo-env] which may be useful for some custom defines.


### `Cargo.toml`

```toml
{{#include cc-defines/Cargo.toml}}
```

### `build.rs`

```rust,ignore
{{#include cc-defines/build.rs}}
```

### `src/welcome.c`

```c
{{#include cc-defines/src/welcome.c}}
```

### `src/main.rs`

```rust,ignore
{{#include cc-defines/src/main.rs}}
```

[cargo-env]: https://doc.rust-lang.org/cargo/reference/environment-variables.html
