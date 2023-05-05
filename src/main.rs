use std::fs::File;
use std::io::prelude::*;
use chrono;

fn main() {
    let local_time = chrono::offset::Local::now();
    let time_str = local_time.format("%Y-%m-%d_%H%M").to_string();
    // println!("{}", time_str);


    //let year = time_str.substring(0, 3);
   // let month = time_str.substring(5, 7);
    // let month
    let file_name = format!("journal/{}.txt", time_str);
    // let full_directory /YYYY/MM/YYYY-MM-DD-HHMM.txt
    let mut file = File::create(&file_name).
        expect("error while creating file");
    file.write_all(time_str.as_bytes()).expect("Unable to write data");
}
