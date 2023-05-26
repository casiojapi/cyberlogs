use std::io::prelude::*;
use chrono;
use std::fs::File;
use std::path::Path;
//use std::env;
use std::fs::OpenOptions;
use dirs;

fn main() {

    // if argc > 1 
    // la idea seria armar un archivo de texto de cada mes
    // usar una db - indexar años, mess 
    //      ek
    // YYYY-MM-DD-HHMM
    let local_time = chrono::offset::Local::now();
    let time_str = local_time.format("%Y-%m-%d-%H%M").to_string();
    let home_dir = dirs::home_dir().expect("could not get home dir");
    let journal_dir = home_dir.join(".journal");


//    let journal_dir = "./journal";
    if let Err(_err) = std::fs::create_dir_all(&journal_dir) {
        //eprintln!("Failed to create directory: {}", err);
    } else {
       // println!("Directory created successfully: {:?}", journal_dir);
    }
    let file_name = &time_str[0..10];
    let hour_min = &time_str[11..15];
//    println!("file_name: {}, hour_min: {}", file_name, hour_min);

    let directory = format!("{}/{}.txt", journal_dir.display(), file_name);
    let file_print = format!("today: {}\n----------------------\n\n\n", time_str);
    
    let path = Path::new(&directory);
    
    if path.exists() {
        let file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(&directory);
        let file_append = format!("\n\nnew log entry at: {}\n----------------------\n\n\n", hour_min);
        file.expect("Error with file").write_all(file_append.as_bytes()).expect("Unable to append data");
    } else {
        let mut file = File::create(&directory).
            expect("error while creating file");
        file.write_all(file_print.as_bytes()).expect("Unable to write data");
    }
    println!("{}", directory);
    
}
