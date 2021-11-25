use std::io;

fn main() {
    println!("This is a stdin test! Please input some words or sentences.");
    let mut buffer = String::new();
    let stdin = io::stdin();
    stdin
        .read_line(&mut buffer)
        .expect("Input error: read_line()");
    println!("Your input is {}", buffer);
}
