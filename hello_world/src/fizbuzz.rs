pub fn fizbuzz(n: i32) -> String {
    let result = match n {
        n if n % 15 == 0 => "FizzBuzz".to_string(),
        n if n % 3 == 0 => "Fizz".to_string(),
        n if n % 5 == 0 => "Buzz".to_string(),
        _ => n.to_string(),
    };
    result
}
