## Using transactions

[![postgres-badge]][postgres] [![cat-database-badge]][cat-database]

[`Client::connect`] will open the local database `cats`, to access the
two tables created in the earlier recipe. The `cat_colors` table must exist.

Begin a transaction with [`Client::transaction`]. Transactions will
roll back unless committed explicitly with [`Transaction::commit`].

In the following example, are colors added to a table having
a unique constraint on the color name. When an attempt to insert
a duplicate color is made, the transaction rolls back.

At the end of `successful_tx()` the `cat_colors` table should have
exactly two rows, with the colors 'lavender' and 'blue'.

At the end of `rolled_back_tx()`, the `cat_colors` table should still
have just 'lavender' and 'blue', since the the `DELETE` and the
`INSERT`s will not have been committed.

```rust,ignore
{{#include examples/pg_transactions.rs}}
```

[`Client::transaction`]: https://docs.rs/postgres/latest/postgres/struct.Client.html#method.transaction
[`Transaction::commit`]: https://docs.rs/postgres/latest/postgres/struct.Transaction.html#method.commit
