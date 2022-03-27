use postgres::{Client, NoTls, Error};

fn main() -> Result<(), Error> {
    let mut client = Client::connect("postgresql://postgres:ChangeMe99@localhost/cats", NoTls)?;

    successful_tx(&mut client)?;
    println!("successful done");

    let res = rolled_back_tx(&mut client);
    assert!(res.is_err());

    // Query data and print.
    println!("Color");
    for row in client.query( "SELECT name from cat_colors", &[])? {
        let name: &str = row.get(0);
        println!("{name}");
    }
    Ok(())
}

fn successful_tx(client: &mut Client) -> Result<(), Error> {
    let mut tx = client.transaction()?;

    tx.batch_execute("DELETE FROM cats")?;
    tx.batch_execute("DELETE FROM cat_colors")?;
    tx.execute("INSERT INTO cat_colors (name) VALUES ($1)", &[&"lavender"])?;
    tx.execute("INSERT INTO cat_colors (name) VALUES ($1)", &[&"blue"])?;
    tx.commit()
}

fn rolled_back_tx(client: &mut Client) -> Result<(), Error> {
    let mut tx = client.transaction()?;

    tx.batch_execute("DELETE FROM cat_colors")?;
    tx.execute("INSERT INTO cat_colors (name) VALUES ($1)", &[&"grey"])?;
    tx.execute("INSERT INTO cat_colors (name) VALUES ($1)", &[&"black"])?;

    // Since this table has a UNIQUE constaint on color names, this will fail.
    tx.execute("INSERT INTO cat_colors (name) VALUES ($1)", &[&"grey"])?;

    tx.commit()
}

