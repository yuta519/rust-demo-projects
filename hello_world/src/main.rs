mod square;
use square::Area;

fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }
}

fn main() {
    let squarere = square::Square::new(10);
    print!("{}", squarere.area());
    print!("\n{}", add(1, 2))
}
