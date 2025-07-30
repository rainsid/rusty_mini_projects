use std::env;
use std::fs;
use std::path::Path;
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
                            println!("Removing metadata:\n\t{s}");
                            remove_metadata(&s);
                            println!("Compressing image:\n\t{s}");
                            compress_image(&s);
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
    Command::new("exiftool")
        .arg("-all=")
        .arg("-overwrite_original")
        .arg(path)
        .output()
        .expect("Metadata removal failed");
}

fn compress_image(path: &String) {
    Command::new("magick")
        .arg(path)
        .arg("-quality")
        .arg("70")
        .arg(path)
        .output()
        .expect("Compressing image failed.");
}
