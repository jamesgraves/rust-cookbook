## Compile and link statically to a bundled C++ library

[![cc-badge]][cc] [![cat-development-tools-badge]][cat-development-tools]

Linking a bundled C++ library is very similar to linking a bundled C library. The two core differences when compiling and statically linking a bundled C++ library are specifying a C++ compiler via the builder method [`cpp(true)`][cc-build-cpp] and preventing name mangling by the C++ compiler by adding the `extern "C"` section at the top of our C++ source file.


### `Cargo.toml`

```toml
{{#include cc-bundled-cpp/Cargo.toml}}
```

### `build.rs`

```rust,no_run
{{#include cc-bundled-cpp/build.rs}}
```
```

### `src/multiply.cpp`

```cpp
{{#include cc-bundled-cpp/src/multiply.cpp}}
```

### `src/main.rs`

```rust,no_run
{{#include cc-bundled-cpp/src/main.rs}}
```

[cc-build-cpp]: https://docs.rs/cc/*/cc/struct.Build.html#method.cpp
