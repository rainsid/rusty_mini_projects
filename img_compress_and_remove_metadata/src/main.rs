use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let dir = &args[1];

    println!("Directory: {dir}");

    match fs::read_dir(dir) {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(entry) => println!("{:?}", entry.path()),
                    Err(e) => eprintln!("Error: {e}"),
                }
            }
        }
        Err(e) => eprintln!("Error: {e}"),
    }
}
