use std::fmt;

#[derive(Copy, Clone, PartialEq, Hash, Debug)]
pub struct Colour {
    r: u8,
    g: u8,
    b: u8,
}

impl Colour {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn tuple(self) -> (u8, u8, u8) {
        (self.r, self.g, self.b)
    }
}


impl fmt::Display for Colour {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{r:02x}{g:02x}{b:02x} ({r}, {g}, {b})", r = self.r, g = self.g, b = self.b)
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_create() {
        let colour = Colour::new(255, 54, 78);
        assert_eq!(colour.tuple(), (255, 54, 78));
        assert_eq!("ff364e (255, 54, 78)", format!("{}", colour));
    }


}
