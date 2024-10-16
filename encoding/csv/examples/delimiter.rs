use std::error::Error;
use serde::Deserialize;
use csv::ReaderBuilder;

#[derive(Debug, Deserialize)]
struct Record {
    name: String,
    place: String,
    #[serde(deserialize_with = "csv::invalid_option")]
    id: Option<u64>,
}

fn main() -> Result<(), Box<dyn Error>> {

    let data = "\
name\tplace\tid
Mark\tMelbourne\t46
Ashley\tZurich\t92
Brian\tVenice\t
";

    let mut reader = ReaderBuilder::new().delimiter(b'\t').from_reader(data.as_bytes());
    for result in reader.deserialize::<Record>() {
        match result {
            Ok(rec) => {
                print!("name: {:10}  place: {:10}", rec.name, rec.place);
                if let Some(id) = rec.id {
                    println!("  id: {id:4}");
                } else  {
                    println!("  id: none");
                }
            },
            Err(e) => {
                println!("Error: {:?}", e);
            },
        }
    }

    Ok(())
}
