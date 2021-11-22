use std::cell::RefCell;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::common::{Colour, Coordinate, Pixel};

pub struct Painter {
    canvas: RefCell<Canvas<Window>>,
}

impl Painter {
    pub fn new(canvas: Canvas<Window>) -> Self {
        Self {
            canvas: RefCell::new(canvas),
        }
    }

    fn draw_square(&self, rectangle: &Rect, colour: Colour) {
        let mut canvas = self.canvas.borrow_mut();
        let (r, g, b) = colour.tuple();
        canvas.set_draw_color(Color::RGB(r, g, b));
        if let Err(err) = canvas.fill_rect(*rectangle) {
            error!("Error drawing a square to the canvas: {}", err);
        }
    }

    fn create_square(size: u32, coordinates: Coordinate) -> Rect {
        let (x, y) = coordinates.tuple();
        debug!("Creating square size: {}, x: {}, y: {}", size, x, y);
        Rect::new(x as i32, y as i32, size, size)
    }

    fn calculate_coordinates(&self, scale: u32, padding: u32, pixel: &Pixel) -> Coordinate {
        let (x, y) = pixel.coordinate().tuple();
        let tx = ((x * scale) + (x * padding)) + (padding / 2);
        let ty = ((y * scale) + (y * padding)) + (padding / 2);
        Coordinate::new(tx, ty)
    }

    pub fn draw(&self,
                scale: u32,
                padding: u32,
                pixel: &Pixel) {
        let coordinates = self.calculate_coordinates(scale, padding, pixel);
        let square = Self::create_square(scale, coordinates);
        self.draw_square(&square, pixel.colour());
        debug!("Drew pixel: {}", pixel);
    }

    pub fn update(&self) {
        let mut canvas = self.canvas.borrow_mut();
        canvas.present();
        debug!("Updated canvas");
    }

    pub fn clear(&self) {
        let mut canvas = self.canvas.borrow_mut();
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        debug!("Cleared canvas");
    }
}