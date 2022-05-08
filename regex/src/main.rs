use regex::Regex;

fn main() {
    check_date("2014-01-01");
    check_date("2022-01-01");
    check_date("2022222222222-01-01");
}

fn check_date(input: &str) -> bool {
    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    print!("The format of `{}` is {}\n", input, re.is_match(input));
    return re.is_match(input);
}

#[test]
fn test_regex_dateformat() {
    assert_eq!(check_date("2022-01-01"), true);
    assert_eq!(check_date("20222222222222-01-01"), false);
}
