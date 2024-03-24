struct User {
    name: String,
    age: u32,
}

pub fn main() {
    let user = User {
        name: "Alice".to_string(),
        age: 30,
    };

    println!("Name: {}, Age: {}", user.name, user.age);
}
