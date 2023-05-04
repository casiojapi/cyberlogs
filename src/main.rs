use std::fs::File;
use std::io::prelude::*;
use chrono;

fn main() {
    let local_time = chrono::offset::Local::now();
    let time_str = local_time.format("%Y-%m-%d-%H%M").to_string();
//    println!("{}", time_str);
    let file_name = format!("{}.txt", time_str);
    let mut file = File::create(&file_name).
        expect("error while creating file");
    file.write_all(time_str.as_bytes()).expect("Unable to write data");
}
