use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process::Command;
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let dir = &args[1];
    let coord_file = &args[2];

    let coordinates = read_coordinates_file(coord_file);

    match fs::read_dir(dir) {
        Ok(entries) => {
            let mut counter = 0;
            for entry in entries {
                match entry {
                    Ok(entry) => {
                        if let Some(s) = path_to_string(&entry.path()) {
                            //geotag here
                            println!("counter:{counter}");
                            geotag(&coordinates[counter], &s);
                            println!("coordinates: {}", &coordinates[counter]);
                            counter += 1;
                        }
                    }

                    Err(e) => eprintln!("Error: {e}"),
                }
            }
        }
        Err(e) => eprintln!("Error: {e}"),
    }
}

fn path_to_string(path: &Path) -> Option<String> {
    path.to_path_buf().into_os_string().into_string().ok()
}

fn read_coordinates_file(file_path: &String) -> Vec<String> {
    let file = File::open(file_path).expect("Could not open file");
    let reader = BufReader::new(file);
    let mut coordinates: Vec<String> = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Could read line");
        coordinates.push(line);
    }

    coordinates
}

fn geotag(coordinates: &String, path: &String) {
    let gps_arg = format!("-gpsposition={coordinates}");

    let status = Command::new("exiftool")
        .arg(gps_arg)
        .arg(path)
        .arg("-overwrite_original")
        .status()
        .expect("Failed to execute command");

    if status.success() {
        println!("Successfully updated GPS posisiton");
    } else {
        eprintln!("exiftool command failed: {status}");
    }
}
