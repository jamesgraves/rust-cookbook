## Create a SQLite database

[![rusqlite-badge]][rusqlite] [![cat-database-badge]][cat-database]

Use the `rusqlite` crate to open SQLite databases. See
[crate][documentation] for compiling on Windows.

[`Connection::open`] will create the database file `cats.db` in the
current working directory if it doesn't already exist.

Then the two tables are created if they don't already exist. The color
of a specific cat must be defined in the `cat_colors` table before a `cat`
row is inserted.


```rust
{{#include examples/initialization.rs}}
```


[`Connection::open`]: https://docs.rs/rusqlite/*/rusqlite/struct.Connection.html#method.open

[documentation]: https://github.com/jgallagher/rusqlite#user-content-notes-on-building-rusqlite-and-libsqlite3-sys
