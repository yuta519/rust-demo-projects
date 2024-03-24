pub struct User {
    name: String,
    age: u32,
}

impl User {
    pub fn new(name: String, age: u32) -> Self {
        Self { name, age }
    }

    pub fn print(&self) {
        println!("Name: {}, Age: {}", &self.name, &self.age);
    }

    pub fn change_name(&mut self, name: String) {
        self.name = name;
    }
}
