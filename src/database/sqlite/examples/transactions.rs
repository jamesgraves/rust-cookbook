use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let mut conn = Connection::open("cats.db")?;

    successful_tx(&mut conn)?;

    let res = rolled_back_tx(&mut conn);
    assert!(res.is_err());

    Ok(())
}

fn successful_tx(conn: &mut Connection) -> Result<()> {
    let tx = conn.transaction()?;

    tx.execute("DELETE FROM cat_colors", [])?;
    tx.execute("INSERT INTO cat_colors (name) VALUES (?1)", &[&"lavender"])?;
    tx.execute("INSERT INTO cat_colors (name) VALUES (?1)", &[&"blue"])?;

    tx.commit()
}

fn rolled_back_tx(conn: &mut Connection) -> Result<()> {
    let tx = conn.transaction()?;

    tx.execute("DELETE FROM cat_colors", [])?;
    tx.execute("INSERT INTO cat_colors (name) VALUES (?1)", &[&"grey"])?;
    tx.execute("INSERT INTO cat_colors (name) VALUES (?1)", &[&"black"])?;

    // Since this table has a UNIQUE constaint on color names, this will fail.
    tx.execute("INSERT INTO cat_colors (name) VALUES (?1)", &[&"grey"])?;

    tx.commit()
}
