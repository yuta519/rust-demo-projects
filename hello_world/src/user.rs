use std::fmt::{Display, Formatter};

pub struct User {
    name: String,
    age: u32,
}

impl Display for User {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "User Name is {}, Age is {}", &self.name, &self.age)
    }
}

impl User {
    pub fn new(name: String, age: u32) -> Self {
        Self { name, age }
    }
}
