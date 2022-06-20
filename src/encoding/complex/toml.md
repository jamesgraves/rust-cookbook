## Deserialize a TOML configuration file

[![toml-badge]][toml] [![cat-encoding-badge]][cat-encoding]

Parse some TOML into a universal `toml::Value` that is able to represent any
valid TOML data.

```rust
{{#include examples/toml.rs}}
```

Parse TOML into your own structs using [Serde].

```rust
{{#include examples/serde-toml.rs}}
```
