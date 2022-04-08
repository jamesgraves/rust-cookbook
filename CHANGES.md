
Changes for the 2022 rewrite / reorginzation / expansion:

1. All examples updated to Rust edition 2021, and the latest stable versions
of the crates.
2. All example code moved to actual `.rs` files. All examples are runnable
inside the project via `cargo run --example`, and 100% of the examples are also
run via `cargo test`. This makes it easier for users to run the examples that are
incompatible with the Rust Playground because they interact with the filesystem,
or a database, etc.. Modifying and running the examples on the command
line shall be encouraged.
3. As a consequence of the above, elimination of the `skeptic` crate to
test code within the Markdown files. Source code for the examples is
brought in by the `{{#include examples/foobar.rs}}` Markdown directive.
4. The code is organized as a series of workspaces. This also avoids the
problem with the `skeptic` based system which would sometimes link in the
incorrect version of a crate for a particular codebase. All the workspaces
share the same target directory, which should reduce compilation times.
5. No hiding of details. All code presented as full and complete.  The
documentation should explain any details needed to understand the example.
6. Remove use of `error-chain` crate. This has been replaced with the
simpler use of `Result<(), Box<dyn Error>>` or similar in the examples.
There will be an extended section on using `anyhow` and `thiserror` as well.
See also: https://nick.groenen.me/posts/rust-error-handling/
