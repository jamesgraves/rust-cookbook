## Insert and Select data

[![rusqlite-badge]][rusqlite] [![cat-database-badge]][cat-database]

[`Connection::open`] will open the database `cats` created in the earlier recipe.

This example inserts data into `cat_colors` and `cats` tables using the [`execute`] method of `Connection`. First, the data is inserted into the `cat_colors` table. After a record for a color is inserted, [`last_insert_rowid`] method of `Connection` is used to get `current_color_row_id` of the last color inserted. This `current_color_row_id` is used while inserting data into the `cats` table. Then, the select query is prepared using the [`prepare`] method which gives a [`statement`] struct.

Then, a query is executed using [`query_map`] method of [`statement`], and then the results are printed.

```rust,ignore
{{#include examples/insert_select.rs}}
```

[`Connection::open`]: https://docs.rs/rusqlite/*/rusqlite/struct.Connection.html#method.open
[`prepare`]: https://docs.rs/rusqlite/*/rusqlite/struct.Connection.html#method.prepare
[`statement`]: https://docs.rs/rusqlite/*/rusqlite/struct.Statement.html
[`query_map`]: https://docs.rs/rusqlite/*/rusqlite/struct.Statement.html#method.query_map
[`execute`]: https://docs.rs/rusqlite/*/rusqlite/struct.Connection.html#method.execute
[`last_insert_rowid`]: https://docs.rs/rusqlite/*/rusqlite/struct.Connection.html#method.last_insert_rowid
