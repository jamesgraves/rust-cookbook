# A Rust Cookbook &emsp; [![Build Status travis]][travis]

[Build Status travis]: https://api.travis-ci.com/jamesgraves/rust-cookbook.svg?branch=main
[travis]: https://travis-ci.com/jamesgraves/rust-cookbook

**[Read it here]**.

This _Rust Cookbook_ is a collection of simple [Rust] examples that
demonstrate good practices to accomplish common programming tasks,
using the crates of the Rust ecosystem.

These examples are complete, and suitable for copying directly into
new cargo projects. They are tested and guaranteed to work.

## Read it offline

If you'd like to read it locally:

```bash
$ git clone https://github.com/jamesgraves/rust-cookbook
$ cd rust-cookbook
$ cargo install mdbook --vers "0.4.15"
$ mdbook serve --open
```

The output can also be opened from the `book` subdirectory in your web browser.

```bash
$ xdg-open ./book/index.html # linux
$ start .\book\index.html    # windows
$ open ./book/index.html     # mac
```

[Read it here]: https://jamesgraves.github.io/rust-cookbook
[Rust]: https://www.rust-lang.org/

## Create a Rust project based on an example

All the examples can be copied out to a new project and modified from there.

```bash
# Create a new project based on a Rayon example
$ cargo new my_new_rayon_project

# Overwrite the main.rs with the example code
$ cp rust-cookbook/src/concurrency/parallel/examples/rayon-map-reduce.rs my_new_rayon_project/src/main.rs

# Add the required libraries as mentioned in the recipe to the new project's Cargo.toml
$ cd my_new_rayon_project/
$ cargo add rayon

# Run it!
$ cargo run
```

## Contributing

This project is intended to be easy for new [Rust] programmers to
contribute to, and an easy way to get involved with the Rust
community. It needs and welcomes help.

For details see [CONTRIBUTING.md](CONTRIBUTING.md) on GitHub.

## License [![CC0-badge]][CC0-deed]

Rust Cookbook is licensed under Creative Commons Zero v1.0 Universal License
([LICENSE-CC0](LICENSE-CC0) or https://creativecommons.org/publicdomain/zero/1.0/legalcode)

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in Rust Cookbook by you, as defined in the CC0-1.0 license, shall be
[dedicated to the public domain][CC0-deed] and licensed as above, without any additional
terms or conditions.

[CC0-deed]: https://creativecommons.org/publicdomain/zero/1.0/deed.en
[CC0-badge]: https://mirrors.creativecommons.org/presskit/buttons/80x15/svg/cc-zero.svg
