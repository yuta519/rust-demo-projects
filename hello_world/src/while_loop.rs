pub fn add_until(start: i32, end: i32) -> i32 {
    let mut sum = 0;
    let mut temp = start;

    while temp <= end {
        sum += temp;
        temp += 1;
    }
    sum
}
