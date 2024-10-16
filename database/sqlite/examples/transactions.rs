use anyhow::{Error, Result};
use rusqlite::Connection;

fn main() -> Result<()> {
    let mut conn = Connection::open("cats.db")?;

    successful_tx(&mut conn)?;
    println!("successful transaction is successful");

    let res = rolled_back_tx(&mut conn);
    assert!(res.is_err());
    println!("rolled back transaction is an error");

    Ok(())
}

fn successful_tx(conn: &mut Connection) -> Result<()> {
    let tx = conn.transaction()?;

    tx.execute("DELETE FROM cat_colors", [])?;
    tx.execute("INSERT INTO cat_colors (name) VALUES (?1)", &[&"lavender"])?;
    tx.execute("INSERT INTO cat_colors (name) VALUES (?1)", &[&"blue"])?;

    tx.commit().map_err(Error::new)
}

fn rolled_back_tx(conn: &mut Connection) -> Result<()> {
    let tx = conn.transaction()?;

    tx.execute("DELETE FROM cat_colors", [])?;
    tx.execute("INSERT INTO cat_colors (name) VALUES (?1)", &[&"grey"])?;
    tx.execute("INSERT INTO cat_colors (name) VALUES (?1)", &[&"black"])?;

    // Since this table has a UNIQUE constaint on color names, this will fail.
    tx.execute("INSERT INTO cat_colors (name) VALUES (?1)", &[&"grey"])?;

    tx.commit()?;
    Ok(())
}
