use std::collections::HashMap;
use postgres::{Client, NoTls, Error};

#[derive(Debug)]
struct Cat {
    name: String,
    color: String,
}

fn main() -> Result<(), Error> {
    let mut client = Client::connect("postgresql://postgres:ChangeMe99@localhost/cats", NoTls)?;

    client.batch_execute("DELETE FROM cats")?;
    client.batch_execute("DELETE FROM cat_colors")?;

    let mut cat_data = HashMap::new();
    cat_data.insert(String::from("Blue"), vec!["Tigger", "Sammy"]);
    cat_data.insert(String::from("Black"), vec!["Oreo", "Biscuit"]);

    for (color, catnames) in &cat_data {
        client.execute("INSERT INTO cat_colors (name) VALUES ($1)", &[&color])?;

        let current_color_row = client.query_one(
            "SELECT id FROM cat_colors WHERE name = $1", &[&color])?;
        let color_row_id: i32 = current_color_row.get(0);

        for cat in catnames {
            client.execute("INSERT INTO cats (name, color_id) VALUES ($1, $2)",
            &[&cat.to_string(),
            &color_row_id])?;
        }
    }

    println!("Cat Name   Color");
    let prepared_stmt = client.prepare("SELECT c.name, cc.name from cats c INNER JOIN cat_colors cc ON cc.id = c.color_id;")?;
    for row in client.query(&prepared_stmt, &[])? {
        let cat = Cat {
            name: row.get(0),
            color: row.get(1),
        };
        println!("{:10} {}", cat.name, cat.color);
    }
    Ok(())
}
