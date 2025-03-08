use rand::Rng;
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
    let pass_length: usize = match args[1].parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Error: Password length must be a number between 8 and 32");
            std::process::exit(1);
        }
    };

    let pass_complexity: u8 = match args[2].parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Error: Password complexity must be a number between 1 and 4");
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

    if !(1..=4).contains(&pass_complexity) {
        eprintln!("Error: Password complexity must be between 1 and 4");
        std::process::exit(1);
    }

    let password = generate_password(&pass_length, &pass_complexity);
    println!("Generated password: {}", password);
}

fn generate_password(length: &usize, &complexity: &u8) -> String {
    let lowercase = "abcdefghijklmnopqrstuvwxyz";
    let uppercase = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let digits = "0123456789";
    let special_chars = "!@#$%^&*()-_=+[]{}|;:,.<>?";

    let mut charset = String::from(lowercase);

    if complexity >= 2 {
        charset.push_str(uppercase);
    }
    if complexity >= 3 {
        charset.push_str(digits);
    }
    if complexity == 4 {
        charset.push_str(special_chars);
    }

    let mut rng = rand::thread_rng();
    let password: String = (0..*length)
        .map(|_| {
            let index = rng.gen_range(0..charset.len());
            charset.chars().nth(index).unwrap()
        })
        .collect();

    password
}
