use std::io;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut buffer)?;
    Ok(())

    // println!("Hello, world!");
    // println!("This is print test by yuta519");
}
