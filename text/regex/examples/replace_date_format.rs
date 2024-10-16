use lazy_static::lazy_static;

use std::borrow::Cow;
use regex::Regex;

fn reformat_dates(before: &str) -> Cow<str> {
    lazy_static! {
        static ref USA_DATE_REGEX : Regex = Regex::new(
            r"(?P<m>\d{2})/(?P<d>\d{2})/(?P<y>\d{4})"
            ).unwrap();
    }
    USA_DATE_REGEX.replace_all(before, "$y-$m-$d")
}

fn main() {
    let before = "apple 03/14/2012, cherry 01/15/2013 and banana 07/05/2014";
    let after = reformat_dates(before);
    assert_eq!(after, "apple 2012-03-14, cherry 2013-01-15 and banana 2014-07-05");
}
