use std::fmt;
use std::str::FromStr;
// Enum <-> string interaction
use strum_macros::EnumString;

use std::fs::File;

use csv;
use csv::StringRecord;

use time::{Date, Time, Month};

use std::panic;


use std::io;

#[derive(strum::Display, EnumString)]
pub enum Tag { Rust, Cpp, Python, Statistics, Finance }

#[derive(strum::Display, EnumString)]
pub enum Category {
    Programming,
    Mathematics
}

pub struct TimeEntry {
    pub category : Category,
    pub tags : Vec<Tag>,
    pub start_date : Date,
    pub start_time : Time,
    pub end_date : Date,
    pub end_time : Time,
    pub description : String
}

impl fmt::Display for TimeEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{},{{{}}},{},{},{},{},{}",
            self.category,
            vec_to_string(&self.tags),
            self.start_date,
            time_to_string(self.start_time),
            self.end_date,
            time_to_string(self.end_time),
            if &self.description != "" { self.description.clone() } else { String::from("\"\"") },
        )
    }
}

fn string_record_to_time_entry(record: &StringRecord) -> Result<TimeEntry, io::Error> {
    let category = Category::from_str(&record[0]).unwrap();
    let tags: Vec<Tag> = vec![Tag::Rust, Tag::Cpp, Tag::Finance];
    let start_date = string_to_date(&record[9]);
    let start_time = string_to_time(&record[10]);
    let end_date = string_to_date(&record[11]);
    let end_time = string_to_time(&record[12]);
    let description = String::from("");

    Ok(TimeEntry {
        category,
        tags,
        start_date,
        start_time,
        end_date,
        end_time,
        description,
    })
}

pub fn csv_to_time_entry(path: &str, ignore_header: bool) -> Result<Vec<TimeEntry>, io::Error> {
    // Opening the file
    let file = if let Ok(x) = File::open(path) {
        x
    } else {
        return Err(io::Error::new(io::ErrorKind::NotFound, "Couldn't open the {path}!"));
    };

    let mut reader = csv::Reader::from_reader(file);

    let mut skipping_row = ignore_header;

    let mut time_entries: Vec<TimeEntry> = Vec::new();
    while let Some(x) = reader.records().next() {
        if skipping_row { // Skips the first line
            skipping_row = false;
        } else {
            if let Ok(record) = x {
                time_entries.push(string_record_to_time_entry(&record).unwrap());

                if let Ok(x) = string_record_to_time_entry(&record) {
                    time_entries.push(x);
                } else {
                    return Err(io::Error::new(io::ErrorKind::InvalidData, "Couldn't convert StringRecord to TimeEntry"));
                }
            } else {
                return Err(io::Error::new(io::ErrorKind::InvalidData, "Couldn't create the StringRecord"));
            }
        }
    }

    Ok(time_entries)
}

fn parse_month(month_str: &str) -> Option<Month> {
    match month_str {
        "01" => Some(Month::January),
        "02" => Some(Month::February),
        "03" => Some(Month::March),
        "04" => Some(Month::April),
        "05" => Some(Month::May),
        "06" => Some(Month::June),
        "07" => Some(Month::July),
        "08" => Some(Month::August),
        "09" => Some(Month::September),
        "10" => Some(Month::October),
        "11" => Some(Month::November),
        "12" => Some(Month::December),
        _ => None,
    }
}

fn string_to_time(str: &str) -> Time {
    // This only works if the string is formatted as HH:MM:SS
    let str_hrs = &str[0..=1];
    let str_min = &str[3..=4];
    let str_sec = &str[6..=7];

    Time::from_hms(str_hrs.parse::<u8>().unwrap(),
        str_min.parse::<u8>().unwrap(),
        str_sec.parse::<u8>().unwrap()).unwrap()
}

fn string_to_date(str: &str) -> Date {
    // This only works if the string is formatted as dd/mm/yyyy
    let str_day = &str[0..=1];
    let day = str_day.parse::<u8>().unwrap();
    let str_month = &str[3..=4];
    let month = if let Some(y) = parse_month(str_month) {
        y
    } else {
        panic!();
    };
    let str_year = &str[6..=9];
    let year = str_year.parse::<i32>().unwrap();
    Date::from_calendar_date(year, month, day).unwrap()
}

fn time_to_string(tme: Time) -> String {
    let (hour, minute, second) = tme.as_hms();

    format!("{:02}:{:02}:{:02}", hour, minute, second)
}

fn vec_to_string<T: ToString>(v: &Vec<T>) -> String {
    let mut ret = String::from("");

    for (index, viter) in v.iter().enumerate() {
        ret.push_str(&viter.to_string());
        
        if index < v.len() - 1 {
            ret.push('_');
        }
    }

    ret
}