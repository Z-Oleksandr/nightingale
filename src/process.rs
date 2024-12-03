use crate::conversion::bytes_to_mb;
use serde::de::value::Error;
use walkdir::WalkDir;
use std::fs::{File, metadata};
use std::io::prelude::*;
use std::collections::HashMap;

pub fn analyze_folder(directory: &str) -> Result<f64, String> {
    println!("Directory: {}", directory);

    let mut output_file = File::create(
        "D:/main_entrance/AppDev/nightingale/output.json"
    ).expect("Error creating output file.");

    let mut size_data = HashMap::new();
    let mut total_size: u64 = 0;

    for entry in WalkDir::new(directory) {
        match entry {
            Ok(entry) => {
                if metadata(entry.path()).expect("Metadata error").is_file() {
                    let file_name = entry.file_name().to_string_lossy().into_owned();
                    let file_size = metadata(entry.path()).unwrap().len();
                    size_data.insert(
                        file_name,
                        file_size
                    );
                    total_size += file_size;
                }
            },
            Err(e) => eprintln!("Error {}", e),
        }
    }
    serde_json::to_writer_pretty(&mut output_file, &size_data)
        .expect("JSONification failed");

    output_file.flush().expect("Failed to flush");

    let total_size_MB: f64 = bytes_to_mb(total_size);

    println!("Total folder size: {:.2} MB", total_size_MB);

    Ok(total_size_MB)
}

pub fn get_full_output() -> String {
    let mut file = File::open(
        "D:/main_entrance/AppDev/nightingale/output.json"
    ).unwrap();

    let mut contents = String::new();

    match file.read_to_string(&mut contents) {
        Ok(_) => contents,
        Err(e) => "Error reading output.".to_owned()
    }
}
