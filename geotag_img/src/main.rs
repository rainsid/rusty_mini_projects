use std::path::Path;
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let dir = &args[1];

    println!("{dir}");

    match fs::read_dir(dir) {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(entry) => {
                        if let Some(s) = path_to_string(&entry.path()) {
                            //geotag here
                            println!("{s}");
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
