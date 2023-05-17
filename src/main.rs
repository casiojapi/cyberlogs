use std::fs::File;
use std::io::prelude::*;
use chrono;
use std::fs;

fn main() {
    let local_time = chrono::offset::Local::now();
    let time_str = local_time.format("%Y-%m-%d-%H%M").to_string();
    //println!("{}", time_str);
    fs::create_dir_all("journal")
        .expect("error creating directory");

    let file_name = format!("journal/{}.txt", time_str);
    let file_print = format!("today: {}\n\n\n", time_str);
    let mut file = File::create(&file_name).
        expect("error while creating file");
    file.write_all(file_print.as_bytes()).expect("Unable to write data");
    println!("{}", file_name);

}
