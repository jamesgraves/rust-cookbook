use postgres::{Client, NoTls, Error};

fn main() -> Result<(), Error> {
    let mut client = Client::connect("postgresql://postgres:ChangeMe99@localhost/cats", NoTls)?;

    client.batch_execute("
        CREATE TABLE IF NOT EXISTS cat_colors (
            id              SERIAL PRIMARY KEY,
            name            VARCHAR UNIQUE NOT NULL
            )
    ")?;

    client.batch_execute("
        CREATE TABLE IF NOT EXISTS cats  (
            id              SERIAL PRIMARY KEY,
            name            VARCHAR NOT NULL,
            color_id        INTEGER NOT NULL REFERENCES cat_colors
            )
    ")?;

    Ok(())
}
