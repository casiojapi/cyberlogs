use std::io::prelude::*;
use chrono;
use std::fs::File;
use std::path::Path;
//use std::env;
use std::fs::OpenOptions;
use dirs;


// formatting for journal files: YYYY-MM-DD-HHMM
fn main() {
    const JOURNAL_PATH: &'static str = ".journal";

    let local_time = chrono::offset::Local::now();

    let time_str = local_time.format("%Y-%m-%d-%H%M").to_string();
    let home_dir = dirs::home_dir().expect("could not get home dir");
    let journal_dir = home_dir.join(JOURNAL_PATH);

    std::fs::create_dir_all(&journal_dir).expect("could not create journal dir");
    
    let file_name = &time_str[0..10];
    let hour_min = &time_str[11..15];

    let log_file_path = format!("{}/{}.md", journal_dir.display(), file_name);
    let file_print = format!("today: {}\n----------------------\n\n\n", time_str);
    
    let path = Path::new(&log_file_path);
    
    if path.exists() {
        let file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(&log_file_path);
        let file_append = format!("\n\nnew log entry at: {}\n----------------------\n\n\n", hour_min);
        file
            .expect("Error with file")
            .write_all(file_append.as_bytes())
            .expect("Unable to append data");
    } else {
        let mut file = File::create(&log_file_path)
            .expect("error while creating file");
        file.write_all(file_print.as_bytes()).expect("Unable to write data");
    }
    println!("{}", log_file_path);
    
}
