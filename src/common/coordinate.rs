use std::fmt;

#[derive(Copy, Clone, PartialEq, Hash, Debug)]
pub struct Coordinate {
    x: u32,
    y: u32,
}

#[allow(dead_code)]
impl Coordinate {
    /// Creates a new coordinate for the given x and y values.
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }

    /// Returns the x value of this Coordinate.
    pub fn x(self) -> u32 {
        self.x
    }

    /// Returns the y value of this Coordinate.
    pub fn y(self) -> u32 {
        self.y
    }

    pub fn tuple(self) -> (u32, u32) {
        (self.x, self.y)
    }
}

impl fmt::Display for Coordinate {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "({}, {})", self.x, self.y)
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_create() {
        let coordinate = Coordinate::new(5, 8);
        assert_eq!(coordinate.tuple(), (5, 8));
        assert_eq!(coordinate.x(), 5);
        assert_eq!(coordinate.y(), 8);
        assert_eq!(coordinate.tuple(), (5, 8));
        assert_eq!("(5, 8)", format!("{}", coordinate));
    }


}

