use rand::Rng;
use std::io;

fn main() {
    println!("This is a stdin test! Guess the number.");
    println!("Please input your guessed number.");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    let mut buffer = String::new();
    let stdin = io::stdin();

    stdin
        .read_line(&mut buffer)
        .expect("Input error: read_line()");

    println!("Your input is {}", buffer);
    println!("Secret is {}", secret_number);
}
