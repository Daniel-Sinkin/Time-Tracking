mod time_entry;
use time_entry::TimeEntry;

use std::fs::File;
use std::io::Write;

fn write_to_file(v: Vec<TimeEntry>, file_name: &str) -> std::io::Result<()> {
    let mut file = File::create(file_name)?;
    for te in v {
        writeln!(file, "{}", te.to_string())?;
    }

    Ok(())
}

fn main() {
    let path = "Data/Clockify.csv";
    let v: Vec<TimeEntry> = time_entry::csv_to_time_entry(path, true).unwrap();

    write_to_file(v, "Output.csv").unwrap();
}