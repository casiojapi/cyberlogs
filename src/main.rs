use std::fs::File;
use std::io::prelude::*;
use chrono;
use std::fs;
use std::path::Path;
use std::fs::OpenOptions;


fn main() {

    // if argc > 1 
    // la idea seria armar un archivo de texto de cada mes
    // usar una db - indexar años, mess 
    //      ek
    // YYYY-MM-DD-HHMM
    let local_time = chrono::offset::Local::now();
    let time_str = local_time.format("%Y-%m-%d-%H%M").to_string();
    fs::create_dir_all("journal")
        .expect("error creating directory");
    let file_name = &time_str[0..10];
    let hour_min = &time_str[11..14];
    println!("file_name: {}, hour_min: {}", file_name, hour_min);

    let directory = format!("journal/{}.txt", file_name);
    let file_print = format!("today: {}\n\n\n", time_str);
    
    let path = Path::new(file_name);
    
    if path.exists() {
        let file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(directory);
        writeln!(file.expect("XD"), "\n\n{}\n\n\n", hour_min);
    } else {
        let file = File::create(&directory).
            expect("error while creating file");
        file.write_all(file_print.as_bytes()).expect("Unable to write data");
    }
    println!("{}", directory);
    
}
