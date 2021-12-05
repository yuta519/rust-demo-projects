use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("This is a stdin test! Guess the number.");
    println!("Please input your guessed number.");

    loop {
        let secret_number = rand::thread_rng().gen_range(1, 101);
        let mut buffer = String::new();
        let stdin = io::stdin();
        stdin
            .read_line(&mut buffer)
            .expect("Input error: read_line()");

        let guess: u32 = buffer.trim().parse().expect("Please input a number");

        println!("Your input is {}", guess);
        println!("Secret is {}", secret_number);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!!"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
