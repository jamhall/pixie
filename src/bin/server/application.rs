use std::process;

use pixie::common::ApplicationError;
use pixie::io::{Command, Server};
use pixie::ui::{Display, DisplayEvent};

use crate::config::ApplicationConfig;
use crate::event_loop::EventLoop;

pub struct Application {
    config: ApplicationConfig,
}

impl Application {
    pub fn new(config: ApplicationConfig) -> Self {
        Self {
            config
        }
    }

    pub async fn run(&self) -> Result<(), ApplicationError> {
        let (width, height, address) = self.config.split();
        let context = sdl2::init().map_err(|_| ApplicationError::Display)?;
        let server = Server::new(address);
        let mut display = Display::new(width, height, &context);
        let mut event_loop = EventLoop::new(30);

        match server.run().await {
            Ok(()) => {
                loop {
                    event_loop.tick();
                    if let Some(event) = display.poll() {
                        match event {
                            DisplayEvent::Quit => {
                                info!("Exciting");
                                process::exit(1);
                            }
                            DisplayEvent::Redraw => {
                                info!("Redraw");
                            }
                            DisplayEvent::Clear => {
                                debug!("Received a clear command");
                                display.clear();
                                display.update();
                            }
                        }
                    }

                    if let Some(event) = server.poll() {
                        match event {
                            Command::Frame(frame) => {
                                debug!("Received a frame command");
                                // render the frame
                                display.draw(&frame);
                                // show the frame
                                display.update();
                            }
                            Command::Clear => {
                                display.clear();
                                display.update();
                            }
                            Command::None => {}
                        }
                    }
                }
            }
            Err(error) => Err(error)
        }
    }
}