use chrono::{NaiveDate, NaiveDateTime};

fn main() {
    if let Some(nov_2017_date) = NaiveDate::from_ymd_opt(2017, 11, 12) {
        if let Some(five_33) = nov_2017_date.and_hms_opt(17, 33, 44) {
            println!(
                "Number of seconds between 1970-01-01 00:00:00 and {} is {}.",
                five_33, five_33.timestamp());
        }
    }

    if let Some(date_time_after_a_billion_seconds) = NaiveDateTime::from_timestamp_opt(1_000_000_000, 0) {
        println!(
            "Date after a billion seconds since 1970-01-01 00:00:00 was {}.",
            date_time_after_a_billion_seconds);
    }
}
