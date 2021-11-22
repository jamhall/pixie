use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::common::{Coordinate, Frame, Pixel};
use crate::ui::painter::Painter;

pub struct RendererConfig {
    width: u32,
    height: u32,
}

impl RendererConfig {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}


pub struct Renderer {
    width: u32,
    height: u32,
    painter: Painter,
}

impl Renderer {
    pub fn new(config: &RendererConfig, canvas: Canvas<Window>) -> Self {
        let width = config.width();
        let height = config.height();
        let painter = Painter::new(canvas);
        Renderer {
            width,
            height,
            painter,
        }
    }

    pub fn draw(&self, frame: &Frame) {
        debug!("Drawing frame rows: {}, columns: {}, padding: {}, pixels: {}", frame.rows(), frame.columns(), frame.padding(), frame.pixels().len());

        let width = self.width - (frame.padding() * frame.columns());
        let height = self.height - (frame.padding() * frame.rows());

        let scale = (width / frame.columns())
            .min(height / frame.rows())
            .max(1);

        frame.pixels()
            .chunks(frame.columns() as usize)
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .map(|(x, column)| {
                        let coordinate = Coordinate::new(x as u32, y as u32);
                        Pixel::new(coordinate, *column)
                    })
                    .collect::<Vec<Pixel>>()
            })
            .for_each(|pixel| {
                self.painter.draw(scale, frame.padding(), &pixel);
            });
    }

    pub fn update(&self) {
        self.painter.update();
    }

    pub fn clear(&self) {
        self.painter.clear();
    }
}
