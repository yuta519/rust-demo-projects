mod user;

fn main() {
    let user = user::User::new("Alice".to_string(), 30);
    print!("{}", user)
}
