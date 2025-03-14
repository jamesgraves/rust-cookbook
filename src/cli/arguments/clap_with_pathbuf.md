## Clap (builder style)

[![clap-badge]][clap] [![cat-command-line-badge]][cat-command-line]

This application describes the structure of its command-line interface using
`clap`'s builder style. The [documentation] gives two other possible ways to
instantiate an application.

The code here requires `clap` version 4. Use `clap = "4"` in
your `Cargo.toml` to get the correct version.

In the builder style, each possible argument is described by an `Arg`
struct. The string given to `Arg::new()` is the internal
name of the argument. The `short` and `long` options control the
flag the user will be expected to type; short flags look like `-f` and long
flags look like `--file`.

The `get_one()` method is used to get an argument's value.
It returns `Some(&`value`)` if the argument was supplied by
the user, else `None`.

The use of `PathBuf` is to allow file paths which are legal
in Linux and MacOS, but not in Rust UTF-8 strings. This is
best practice: one encounters such paths quite rarely in
practice, but when it happens it is really frustrating
without this.

```rust
{{#include examples/clap_with_pathbuf.rs}}
```

Usage information is generated by `clap -h`. The usage for
the example application looks like this.

```
Teaches argument parsing

Usage: clap-cookbook [OPTIONS]

Options:
  -f, --file <file>   A cool file
  -n, --number <num>  Five less than your favorite number
  -h, --help          Print help
  -V, --version       Print version
```

We can test the application by running a command like the following.

```
$ cargo run --example clap_with_pathbuf.rs -- -f myfile.txt -n 251
```

The output is:

```
The file passed is: myfile.txt
Your favorite number must be 256.
```

[documentation]: https://docs.rs/clap/
