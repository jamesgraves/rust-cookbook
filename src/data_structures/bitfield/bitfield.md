## Define and operate on a type represented as a bitfield

[![bitflags-badge]][bitflags] [![cat-no-std-badge]][cat-no-std]

Add the `bitflags` crate to your own project:

```
cargo add bitflags
```

Creates type safe bitfield type `MyFlags` with help of [`bitflags!`] macro
and implements elementary `clear` operation as well as [`Display`] trait for it.
Subsequently, shows basic bitwise operations and formatting.

```rust
{{#include examples/bitfield.rs}}
```

[`bitflags!`]: https://docs.rs/bitflags/*/bitflags/macro.bitflags.html
[`Display`]: https://doc.rust-lang.org/std/fmt/trait.Display.html
