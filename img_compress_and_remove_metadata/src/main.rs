use core::panic::PanicInfo;
use std::env;
use std::ffi::os_str;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();
    let dir = &args[1];

    println!("Directory: {dir}");

    match fs::read_dir(dir) {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(entry) => {
                        if let Some(s) = path_to_string(&entry.path()) {
                            println!("{s}");
                            remove_metadata(&s);
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

fn remove_metadata(path: &String) {
    println!("{path}");
}
