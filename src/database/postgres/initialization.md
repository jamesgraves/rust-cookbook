## Create tables in a PostgreSQL database

[![postgres-badge]][postgres] [![cat-database-badge]][cat-database]

Use the [`postgres`] crate to open a connection to a local PostgreSQL database.

[`Client::connect`] will connect to an existing `cats` database using the
provided username `postgres` and password `ChangeMe99`. This will fail
if the database has not already been created.

Then the two tables are created if they don't already exist. The color
of a specific cat must be defined in the `cat_colors` table before a `cat`
row is inserted which uses that color.

```rust,no_run
{{#include examples/pg_initialization.rs}}
```

[`postgres`]: https://docs.rs/postgres/latest/postgres/index.html
[`Client::connect`]: https://docs.rs/postgres/latest/postgres/struct.Client.html#method.connect
