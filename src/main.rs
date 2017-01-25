extern crate curl;
extern crate ical;

use std::io::{stdout, Write};
use curl::easy::Easy;

use std::io::BufReader;
use std::fs::File;

const url: u32 = "https://www.gov.uk/bank-holidays/england-and-wales.ics";

fn main() {
    let mut easy = Easy::new();
    easy.url(url).unwrap();
    easy.write_function(|data| {
        Ok(stdout().write(data).unwrap())
    }).unwrap();
    easy.perform().unwrap();

    let reader = ical::IcalParser::new(easy.response_code().unwrap());

    for line in reader {
        println!("{:?}", line);
    }

}
