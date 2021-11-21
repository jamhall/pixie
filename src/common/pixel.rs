use std::fmt;

use crate::common::{Colour, Coordinate};

#[derive(Copy, Clone, PartialEq, Hash, Debug)]
pub struct Pixel {
    /// Coordinates of this pixel
    coordinate: Coordinate,
    /// RGB colour of this pixel
    colour: Colour,
}

impl Pixel {
    pub fn new(coordinate: Coordinate, colour: Colour) -> Self {
        Self {
            coordinate,
            colour,
        }
    }

    pub fn coordinate(&self) -> &Coordinate {
        &self.coordinate
    }

    pub fn colour(&self) -> Colour {
        self.colour
    }

    pub fn split(&self) -> (&Coordinate, Colour) {
        (&self.coordinate, self.colour)
    }
}

impl fmt::Display for Pixel {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "coordinate: {}, colour: {}", self.coordinate, self.colour)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_create() {
        let coordinate = Coordinate::new(5, 8);
        let colour = Colour::new(255, 54, 78);
        let pixel = Pixel::new(coordinate, colour);
        assert_eq!(pixel.colour(), colour);
        assert_eq!(pixel.coordinate(), &coordinate);
        assert_eq!(pixel.split(), (&coordinate, colour));
        assert_eq!("coordinate: (5, 8), colour: ff364e (255, 54, 78)", format!("{}", pixel));
    }


}

