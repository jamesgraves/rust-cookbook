use std::error::Error;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Record {
    name: String,
    place: String,
    #[serde(deserialize_with = "csv::invalid_option")]
    id: Option<u64>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = "\
name,place,id
doug,phoenix,
mark,sydney,46.5
ashley,zurich,92
akshat,delhi,37
alisha,colombo,xyz
";

    let mut rdr = csv::Reader::from_reader(data.as_bytes());
    for result in rdr.deserialize() {
        let record: Record = result?;
        // println!("{:?}", record);
        print!("{:10}  {:10}  ", record.name, record.place);
        if let Some(id) = record.id {
            println!("{id:2}");
        } else {
            println!("no id");
        }
    }

    Ok(())
}
