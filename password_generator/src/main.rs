use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        // print error message and exit
        eprintln!("Error: Please provide two argumenst: <length> <complexity> ");
        eprintln!("Usage: ./password_generator <length> <complexity>");
        std::process::exit(1);
    }

    //validate input
    // args[0] is the program path
    let pass_length: u8 = match args[1].parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Error: Password length must be a number between 8 and 32");
            std::process::exit(1);
        }
    };

    let pass_complexity: u8 = match args[2].parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Error: Password complexity must be a number between 1 and 3");
            std::process::exit(1);
        }
    };

    // if _pass_length > 32 || _pass_length < 8 {
    //     eprintln!("Error: Password length must be between 8 and 32");
    //     std::process::exit(1);
    // }
    //
    // if _pass_complexity > 3 || _pass_complexity < 1 {
    //     eprintln!("Error: Password complexity must be between 1 and 3");
    //     std::process::exit(1);
    // }
    // println!("{}, {}", pass_length, pass_complexity);
    //
    //   I want to be idiomatic here. Rust analyzer (Clippy) keeps complaining

    if !(8..=32).contains(&pass_length) {
        eprintln!("Error: Password length must be between 8 and 32");
        std::process::exit(1);
    }

    if !(1..=3).contains(&pass_complexity) {
        eprintln!("Error: Password complexity must be between 2 and 3");
        std::process::exit(1);
    }
}
