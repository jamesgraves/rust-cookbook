
Operating system packages are required to compile and run the SQLite3 examples
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

Add these crates to your own project:

```
cargo add anyhow rusqlite
```

[`rusqlite`]:  https://docs.rs/rusqlite/*/rusqlite
