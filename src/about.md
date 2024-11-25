# About "Cookin' with Rust"

## Table of contents

- [Who this book is for](#who-this-book-is-for)
- [How to read this book](#how-to-read-this-book)
- [How to use the recipes](#how-to-use-the-recipes)
- [A note about error handling](#a-note-about-error-handling)
- [A note about crate representation](#a-note-about-crate-representation)

## Who this book is for

This cookbook is intended for new Rust programmers, so that they may
quickly get an overview of the capabilities of the Rust crate
ecosystem. It is also intended for experienced Rust programmers, who
should find in the recipes an easy reminder of how to accomplish
common tasks.

## How to read this book

The cookbook [index] contains the full list of recipes, organized into
a number of sections: "database", "encoding", "concurrency", etc.  The
sections themselves are more or less ordered in progression, with
later sections being more advanced, and occasionally building on
concepts from earlier sections.

Within the index, each section contains a list of recipes. The recipes
are simple statements of a task to accomplish, like "generate random
numbers in a range"; and each recipe is tagged with badges indicating
which _crates_ they use, like [![rand-badge]][rand], and which
categories on [crates.io] those crates belong to, like
[![cat-science-badge]][cat-science].

Many of the examples can be run in the [Rust Playground][rust-playground]
directly from the Cookbook, these examples will have a "play" button
in the top right corner. However, some examples require access to the
filesystem, or require external software, so they can only be run
when the Cookbook repository itself is cloned.

New Rust programmers should be comfortable reading from the first
section to the last, and doing so should give one a strong overview of
the crate ecosystem. Click on the section header in the index, or in
the sidebar to navigate to the page for that section of the book.

## How to use the recipes

Recipes are designed to give you instant access to working code, along
with a full explanation of what it is doing, and to guide you to
further information.

All recipes in the cookbook are full, self contained programs, so
that they may be copied directly into your own projects for
experimentation. The instructions to do so are below.

The crate badges that accompany the examples link to the crates' full
documentation on [docs.rs], and is often the next documentation you
should read after deciding which crate suites your purpose.

### Clone the Rust Cookbook

Since all the examples can be run from the Rust Cookbook source
repository, we can just clone it:

```sh
git clone https://github.com/jamesgraves/rust-cookbook.git
```

Next, we will go into the repo and run an example (in this case,
from the randomness section):

```sh
cd rust-cookbook
cargo run --example basic_usage
```

The output will be similar to this:

```
    Finished dev [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/examples/basic_usage`
Random u8: 153
Random u16: 52056
Random u32: 227049851
Random i32: 364271356
Random float: 0.3863679947558778
```

Note that it doesn't matter what the current directory is inside the
Cookbook project, individual examples will run via their unique names.

The cookbook is organized as a series of [`cargo workspaces`], so we can
go into one of them and run unit tests, which run all the examples in that
section:

```sh
cd src/algorithms/randomness
cargo test
```

The individual examples are in the `examples` subdirectory of
`src/algorithms/randomness`, and can easily be
modified and run, either via `cargo run --example` or `cargo test`.

The unit tests run each example as a separate program. We can run the unit
tests for the **entire** cookbook, but some of the tests will fail unless
all the required external programs and libraries are installed.

```sh
# Go to the top level of the rust cookbook:
cd ~/rust-cookbook
cargo test
```

For instance,
the database interface examples require the database software be installed, and
in the case of PostgreSQL, the database server to be configured; see those
sections of the cookbook for instructions to do that.

We can also create an entirely new Rust project based around the example.

```sh
cd ~
cargo new --bin random_basic_usage
```

Now copy over the newly-generated `main.rs` (created by the `cargo new` command)
with the code from the example:

```sh
cp rust-cookbook/src/algorithms/randomness/examples/basic_usage.rs random_basic_usage/src/main.rs
```

Add the `rand` crate to the newly created project:

```sh
cd random_basic_usage
cargo add rand
```

That will modify the `Cargo.toml` which lists the top-level dependencies of the project
we have just created.

Since we have already replaced the `main.rs` with the example code, we can now
compile and run it:

```sh
cargo run
```

And the output should be the same as when we ran the example from inside the Rust Cookbook
repository, as seen above.


## A note about error handling

Error handling in Rust is robust when done correctly, but in today's
Rust it requires a fair bit of boilerplate. Because of this one often
sees Rust examples filled with `unwrap` calls instead of proper error
handling.

Since these recipes are intended to be reused as-is and encourage best
practices, they set up error handling correctly when there are
`Result` types involved.

The basic pattern we use is to have a `fn main() -> Result<(), Box<dyn Error>>`.

The structure generally looks like:

```rust
use std::error::Error;
use std::net::IpAddr;
use std::str;

fn main() -> Result<(), Box<dyn Error>> {
    let bytes = b"2001:db8::1";

    // Bytes to string.
    let s = str::from_utf8(bytes)?;

    // String to IP address.
    let addr: IpAddr = s.parse()?;

    println!("{:?}", addr);
    Ok(())
}

```

By using `Box<dyn Error>`, this allows automatic conversions from two standard
library error types to make the `?` operator work.

For more background on error handling in Rust, read [this page of the
Rust book][error-docs] and [this blog post][error-blog].

## A note about crate representation

We are open to including any library crate that is at least moderately
popular. For example, any library crate mentioned on
[Awesome Rust][awesome-rust-libs] or [blessed.rs][blessed-rs] shall be
considered for inclusion.

Here are some Criteria for *not* including a library:

* The library is still in rapid development, with frequent API-breaking changes.
* The library is not widely used, or likely to only appeal to a very narrow audience.
    * For an example, a crate that is used only for a specific video game.
* Libraries that reqire a large amount of code for a *minimally* useful example.

{{#include links.md}}

[index]: intro.html
[rust-playground]: https://play.rust-lang.org/
[error-docs]: https://doc.rust-lang.org/book/error-handling.html
[error-blog]: https://nick.groenen.me/posts/rust-error-handling/
[crates.io]: https://crates.io
[docs.rs]: https://docs.rs
[Cargo.toml]: http://doc.crates.io/manifest.html
[`cargo-edit`]: https://github.com/killercup/cargo-edit
[awesome-rust-libs]: https://github.com/rust-unofficial/awesome-rust#libraries
[blessed-rs]: https://blessed.rs
