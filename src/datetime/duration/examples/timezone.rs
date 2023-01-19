use chrono::{DateTime, FixedOffset, Local, Utc};

fn main() {
    let local_time = Local::now();
    let utc_time = DateTime::<Utc>::from_utc(local_time.naive_utc(), Utc);
    println!("Local time now is {}", local_time);
    println!("UTC time now is {}", utc_time);
    if let Some(china_timezone) = FixedOffset::east_opt(8 * 3600) {
        println!(
            "Time in Hong Kong now is {}",
            utc_time.with_timezone(&china_timezone)
            );
    }
    if let Some(rio_timezone) = FixedOffset::west_opt(2 * 3600) {
        println!("Time in Rio de Janeiro now is {}", utc_time.with_timezone(&rio_timezone));
    }
}
