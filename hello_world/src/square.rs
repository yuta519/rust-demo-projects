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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_area() {
        let square = Square::new(10);
        assert_eq!(square.area(), 100);
    }
}
