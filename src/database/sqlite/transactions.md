## Using transactions

[![rusqlite-badge]][rusqlite] [![cat-database-badge]][cat-database]

[`Connection::open`] will open the `cats.db` database from the top recipe, and
assumes the `cat_colors` table exists.

Begin a transaction with [`Connection::transaction`]. Transactions will
roll back unless committed explicitly with [`Transaction::commit`].

In the following example, colors are added to a table having
a unique constraint on the color name. When an attempt to insert
a duplicate color is made, the transaction rolls back.

At the end of `successful_tx()` the `cat_colors` table should have
exactly two rows, with the colors 'lavender' and 'blue'.

At the end of `rolled_back_tx()`, the `cat_colors` table should still
have just 'lavender' and 'blue', since the the `DELETE` and the
`INSERT`s will not have been committed.

```rust,ignore
{{#include examples/transactions.rs}}
```

[`Connection::transaction`]: https://docs.rs/rusqlite/*/rusqlite/struct.Connection.html#method.transaction
[`Transaction::commit`]: https://docs.rs/rusqlite/*/rusqlite/struct.Transaction.html#method.commit
