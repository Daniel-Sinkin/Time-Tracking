use time::{Date, Time, Month};

use csv::Reader;

use std::{fs::File, str::FromStr};

use strum_macros::EnumString;

#[derive(strum::Display, EnumString)]
enum Category {
    Programming,
    Mathematics
}

#[derive(strum::Display, EnumString)]
enum Tag { Rust, Cpp, Python, Statistics, Finance }

struct TimeEntry {
    category : Category,
    tags : Vec<Tag>,
    start_date : Date,
    start_time : Time,
    end_date : Date,
    end_time : Time,
    description : String
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

fn string_to_time(str: String) -> Time {
    // This only works if the string is formatted as HH:MM:SS
    let str_hrs = &str[0..=1];
    let str_min = &str[3..=4];
    let str_sec = &str[6..=7];

    Time::from_hms(str_hrs.parse::<u8>().unwrap(),
        str_min.parse::<u8>().unwrap(),
        str_sec.parse::<u8>().unwrap()).unwrap()
}

fn string_to_date(str: String) -> Date {
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

fn main() {
    println!("Hello, World!");

    // Category, Tags, Task, startDate, startTime, endDate, endTime, Description / Notes

    let path = "Data/Clockify.csv";
    let file = File::open(path).unwrap();

    let mut reader = Reader::from_reader(file);

    let mut is_header = true;

    while let Some(x) = reader.records().next() {
        if is_header {
            is_header = false;
        } else {
            if let Ok(y) = x {
                let te = TimeEntry{
                    category: Category::from_str(&y[0]).unwrap(),
                    tags: vec![],
                    start_date: string_to_date(String::from(&y[9])),
                    start_time: string_to_time(String::from(&y[10])),
                    end_date: string_to_date(String::from(&y[11])),
                    end_time: string_to_time(String::from(&y[12])),
                    description: String::from(""),
                };
            }
        }
    }
}