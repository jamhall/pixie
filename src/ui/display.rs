use crate::common::Frame;
use crate::ui::input::{DisplayEvent, Input};
use crate::ui::renderer::Renderer;
use crate::ui::RendererConfig;

pub struct Display {
    renderer: Renderer,
    input: Input,
}

impl Display {
    pub fn new(width: u32, height: u32, context: &sdl2::Sdl) -> Self {
        info!("Creating display with dimensions ({}x{})", width, height);

        let video = context.video().unwrap();
        let window = video
            .window("pixel", width, height)
            .position_centered()
            .build()
            .expect("Could not initialise window");

        let canvas = window
            .into_canvas()
            .build()
            .expect("Could not initialise canvas");

        let renderer_config = RendererConfig::new(width, height);
        let renderer = Renderer::new(&renderer_config, canvas);
        let input = Input::new(context);
        Display { renderer, input }
    }

    pub fn poll(&mut self) -> Option<DisplayEvent> {
        self.input.poll()
    }

    pub fn draw(&self, frame: &Frame) {
        self.renderer.clear();
        self.renderer.draw(frame);
    }

    pub fn clear(&self) {
        self.renderer.clear();
    }

    pub fn update(&self) {
        self.renderer.update();
    }
}