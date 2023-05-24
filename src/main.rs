mod time_entry;
use time_entry::TimeEntry;

use std::fs::File;
use std::io::Write;

use eframe::egui;

fn write_to_file(v: Vec<TimeEntry>, file_name: &str) -> std::io::Result<()> {
    let mut file = File::create(file_name)?;
    for te in v {
        writeln!(file, "{}", te.to_string())?;
    }

    Ok(())
}

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with 'RUST_LOG=debug')
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(640.0, 480.0)),
        ..Default::default()
    };
    eframe::run_native("Tempi", options, Box::new(|_cc| Box::<MyApp>::default()))

    //let path = "Data/Clockify.csv";
    //let v: Vec<TimeEntry> = time_entry::csv_to_time_entry(path, true).unwrap();
    //write_to_file(v, "Output.csv").unwrap();
}

struct MyApp {
    name: String,
    age: u32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "Dnaiel".to_owned(),
            age: 26,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Time Tracking");
        });
    }
}
