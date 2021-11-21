use std::fmt;

use crate::common::Pixel;

#[derive(Debug, Clone)]
pub struct Frame {
    rows: u32,
    columns: u32,
    padding: u32,
    pixels: Vec<Pixel>,
}

impl Frame {
    pub fn new(rows: u32,
               columns: u32,
               padding: u32,
               pixels: Vec<Pixel>) -> Self {
        Self {
            rows,
            columns,
            padding,
            pixels,
        }
    }

    pub fn pixels(&self) -> &Vec<Pixel> {
        &self.pixels
    }

    pub fn rows(&self) -> u32 {
        self.rows
    }

    pub fn columns(&self) -> u32 {
        self.columns
    }

    pub fn padding(&self) -> u32 {
        self.padding
    }
}


impl fmt::Display for Frame {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "rows = {}, columns = {}, padding = {}, pixels = {}",
               self.rows, self.columns, self.padding, self.pixels.len())
    }
}


#[cfg(test)]
mod test {
    use crate::common::{Colour, Coordinate};

    use super::*;

    #[test]
    fn test_create() {
        // @TODO Implement a macro for defining a pixel
        let coordinate = Coordinate::new(5, 8);
        let colour = Colour::new(255, 54, 78);
        let pixel = Pixel::new(coordinate, colour);
        let pixels = vec![pixel];
        let frame = Frame::new(2, 3, 10, pixels);
        assert_eq!(frame.rows(), 2);
        assert_eq!(frame.columns(), 3);
        assert_eq!(frame.padding(), 10);
        assert_eq!(frame.pixels().len(), 1);
        assert_eq!("rows = 2, columns = 3, padding = 10, pixels = 1", format!("{}", frame));
    }
}