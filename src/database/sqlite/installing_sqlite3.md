
Operating system packages are required to compile and run the Sqlite3 examples
which use the [`rusqlite`] crate.

# Debian / Ubuntu

```sh
sudo apt install sqlite3 libsqlite3-dev
```

# Fedora

```sh
sudo dnf install sqlite sqlite-devel
```

-----

Add these crates to your own project with [`cargo-edit`]:

```
cargo add anyhow rusqlite
```

[`cargo-edit`]: https://crates.io/crates/cargo-edit
[`rusqlite`]:  https://docs.rs/rusqlite/*/rusqlite
