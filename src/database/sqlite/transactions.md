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

Since this program uses [`anyhow`] for error handling, the
`rustqlite::Result` returned by the `tx.commit()` must be converted
to a `anyhow::Result`. This can be done in a couple different ways
as shown by the two transaction functions, and they are both equivalent
to this code:

```rust,norun
match tx.commit() {
    Ok(x) => Ok(x),
    Err(err) => Err(anyhow::Error::new(err)),
}
```

[`Connection::transaction`]: https://docs.rs/rusqlite/*/rusqlite/struct.Connection.html#method.transaction
[`Transaction::commit`]: https://docs.rs/rusqlite/*/rusqlite/struct.Transaction.html#method.commit
[`anyhow`]: https://docs.rs/anyhow/*/anyhow/index.html
