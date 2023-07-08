use std::io::{BufReader, BufRead};
use regex::RegexSetBuilder;
use std::error::Error;

use stringreader::StringReader;

fn main() -> Result<(), Box<dyn Error>> {

    let simulated_file_contents = r#"
        WARNING: Timeout Expired!
        192.168.0.1:443
        https://10.18.77.106:443
        version "1.2.3"
    "#;
    let str_reader = StringReader::new(simulated_file_contents);
    let buffered = BufReader::new(str_reader);
    //
    // To filter from a file on the filesystem instead:
    //
    // use std::fs::File;
    // let buffered = BufReader::new(File::open("my_logfile.txt)?);
    // 

    let set = RegexSetBuilder::new(&[
        r#"version "\d\.\d\.\d""#,
        r#"\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}:443"#,
        r#"warning.*timeout expired"#,
    ]).case_insensitive(true)
        .build()?;

    buffered
        .lines()
        .filter_map(|line| line.ok())
        .filter(|line| set.is_match(line.as_str()))
        .for_each(|x| println!("{}", x));

    Ok(())
}
