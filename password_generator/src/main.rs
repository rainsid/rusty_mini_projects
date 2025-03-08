use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // args[0] is the program path
    let pass_length = &args[1];
    let pass_complexity = &args[2];

    println!("{}, {}", pass_length, pass_complexity);
}
