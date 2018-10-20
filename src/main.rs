extern crate chrono;
#[macro_use] extern crate failure;
#[macro_use] extern crate lazy_static;


static CURRENT_MILLENIUM: i32 = 2000;

use regex::Regex;
use chrono::{NaiveDate, NaiveTime, NaiveDateTime};
use std::io::{self, BufRead};
use failure::Error;

fn do_line(line: &str) -> Result<String, Error> {
    let line = line
        .replace("\u{202a}", "")
        .replace("\u{202c}", "");
    let line = line.split(" - ")
        .collect::<Vec<_>>();
    if line.len() == 1 {
        Err(format_err!("failed to split on dash"))?;
    }
    lazy_static! {
        static ref DT_REGEX: Regex = Regex::new(r"(\d{2})\.(\d{2})\.(\d{2}), (\d{2}):(\d{2})").unwrap();
    }
    let datetime = if let Some(cap) = DT_REGEX.captures_iter(&line[0]).next() {
        let yy: i32 = cap[3].parse()?;
        let d = NaiveDate::from_ymd(CURRENT_MILLENIUM + yy, cap[2].parse()?, cap[1].parse()?);
        let t = NaiveTime::from_hms(cap[4].parse()?, cap[5].parse()?, 0);
        NaiveDateTime::new(d, t)
    }
    else {
        return Err(format_err!("failed to extract datetime from: {}", &line[0]));
    };
    let ts = datetime.format("%Y-%m-%d %H:%M:%S");
    let rightmost = line[1].split(": ")
        .collect::<Vec<_>>();
    if rightmost.len() == 1 {
        Ok(format!("{}\t--\t{}", ts, &rightmost[0]))
    }
    else {
        Ok(format!("{}\t{}\t{}", ts, &rightmost[0], &rightmost[1]))
    }
}
fn main() {
    eprintln!("[*] eta's hacky whatsapp message converter; please go on");
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        match do_line(&line) {
            Ok(l) => println!("{}", l),
            Err(e) => {
                eprintln!("[*]: Error processing line '{}': {}", line, e);
            }
        }
    }
}
