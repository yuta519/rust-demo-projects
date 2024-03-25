mod square;
use square::Area;

fn main() {
    let squarere = square::Square::new(10);
    print!("{}", squarere.area());
}
