use time::{Date, Time};

use csv::Reader;

use std::fs::File;

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

fn main() {
    println!("Hello, World!");

    // Category, Tags, Task, startDate, startTime, endDate, endTime, Description / Notes

    let path = "Data/Clockify.csv";
    let file = File::open(path).unwrap();

    let mut reader = Reader::from_reader(file);

    for result in reader.records() {
        match result {
            Ok(record) => {
                for (idx, field) in record.iter().enumerate() {
                    println!("Field {}: {}", idx, field);
                }
            },
            Err(err) => println!("Error reading CSV record: {}", err),
        }
    }
}