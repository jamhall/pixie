use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub struct Input {
    events: sdl2::EventPump,
}

pub enum DisplayEvent {
    Quit,
    Redraw,
    Clear,
}

impl Input {
    pub fn new(sdl_context: &sdl2::Sdl) -> Self {
        Input { events: sdl_context.event_pump().unwrap() }
    }

    pub fn poll(&mut self) -> Option<DisplayEvent> {
        if let Some(event) = self.events.poll_event() {
            return match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    Some(DisplayEvent::Quit)
                }
                Event::KeyDown { keycode: Some(Keycode::C), .. } => {
                    Some(DisplayEvent::Clear)
                }
                Event::KeyDown { keycode: Some(Keycode::R), .. } => {
                    Some(DisplayEvent::Redraw)
                }
                _ => None
            };
        }
        None
    }
}