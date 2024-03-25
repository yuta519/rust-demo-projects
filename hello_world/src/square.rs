pub trait Area {
    fn area(&self) -> u32;
}

pub struct Square {
    side: u32,
}

impl Area for Square {
    fn area(&self) -> u32 {
        self.side.pow(2)
    }
}

impl Square {
    pub fn new(side: u32) -> Self {
        Self { side }
    }
    // pub fn area(&self) -> u32 {
    //     self.side.pow(2)
    // }
}

// fn main() {
//     let squarere = Square::new(10);
//     print!("{}", squarere.area());
// }
