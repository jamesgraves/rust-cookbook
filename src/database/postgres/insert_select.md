## Insert and Select data

[![postgres-badge]][postgres] [![cat-database-badge]][cat-database]

The example will connect to the database, insert some cat data, and then
print the result.

```rust,no_run
{{#include examples/insert_select.rs}}
```

[`Client::connect`] will open the local database `cats`, to access the two tables created in the earlier recipe.

First, all data is cleared out of the `cats` and `cat_colors` tables
using [`batch_execute`]. Data *must* be deleted from `cats` first, as
there is a foreign key dependency upon the `cat_colors` table.

Then the sample data is inserted into `cat_colors` and `cats` tables
using the [`execute`] method of `Client`.

Arguments passed to the SQL statement are $1, $2, $3, etc. which
correspond to the items in the slice reference. Note that the
parenthesis `()` are part of the SQL `INSERT` statement syntax.

First, a new color is inserted into the `cat_colors` table. After a
record for a color is inserted, a query is performed to get the row id
of that color using [`query_one`]. Then this `color_row_id` is used
to reference the new color while inserting data into the `cats` table.

Finally, a prepared statement query is created using the [`prepare`]
method which gives a [`statement`] struct. The query is then executed
using [`query`] method of [`statement`] to print the result. A slice
reference containing the arguments to be passed to the SQL is required
for the call to [`query`], but since no arguments were needed for that
query, the empty slice reference `&[]` is used.

[`Client::connect`]: https://docs.rs/postgres/latest/postgres/struct.Client.html#method.connect
[`batch_execute`]: https://docs.rs/postgres/latest/postgres/struct.Client.html#method.batch_execute
[`execute`]: https://docs.rs/postgres/latest/postgres/struct.Client.html#method.execute
[`prepare`]: https://docs.rs/postgres/latest/postgres/struct.Client.html#method.prepare
[`query`]: https://docs.rs/postgres/latest/postgres/struct.Client.html#method.query
[`query_one`]: https://docs.rs/postgres/latest/postgres/struct.Client.html#method.query_one
[`statement`]: https://docs.rs/postgres/latest/postgres/struct.Statement.html
