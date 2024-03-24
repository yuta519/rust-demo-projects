mod user;

fn main() {
    let mut user = user::User::new("Alice".to_string(), 30);
    user.print();
    user.change_name("Bob".to_string());
    user.print();
}
