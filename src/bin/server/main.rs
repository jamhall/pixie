#![warn(clippy::all)]


#[macro_use]
extern crate log;


use env_logger::Env;

use pixie::common::ApplicationError;

use crate::application::Application;

mod application;
mod cli;
mod config;
mod event_loop;

#[tokio::main]
async fn main() -> Result<(), ApplicationError> {
    let env = Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info");
    env_logger::init_from_env(env);

    info!("Starting display...");

    return match cli::parse() {
        Ok(config) => {
            let application = Application::new(config);
            application.run().await
        }
        Err(error) => Err(error)
    };
}


